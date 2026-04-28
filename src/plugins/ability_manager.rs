use std::collections::HashMap;
use std::fs;
use std::path::PathBuf;
use std::io;

use super::types::{Ability, AbilitySource};

pub struct AbilityManager {
    abilities: HashMap<String, Ability>,
    ability_dir: PathBuf,
    project_ability_dir: PathBuf,
}

impl AbilityManager {
    pub fn new() -> Self {
        let ability_dir = Self::get_default_ability_dir();
        let project_ability_dir = PathBuf::from(".coderex").join("abilities");

        Self {
            abilities: HashMap::new(),
            ability_dir,
            project_ability_dir,
        }
    }

    fn get_default_ability_dir() -> PathBuf {
        if let Some(home) = std::env::var_os("HOME").or_else(|| std::env::var_os("USERPROFILE")) {
            PathBuf::from(home).join(".coderex").join("abilities")
        } else {
            PathBuf::from(".coderex").join("abilities")
        }
    }

    pub fn initialize(&mut self) -> io::Result<()> {
        fs::create_dir_all(&self.ability_dir)?;
        fs::create_dir_all(&self.project_ability_dir)?;

        self.load_builtin_abilities();
        
        let user_dir = self.ability_dir.clone();
        let project_dir = self.project_ability_dir.clone();
        
        self.load_abilities_from_dir(&user_dir, AbilitySource::User)?;
        self.load_abilities_from_dir(&project_dir, AbilitySource::Project)?;

        Ok(())
    }

    fn load_builtin_abilities(&mut self) {
        let builtin_abilities = vec![
            Ability::builtin(
                "commit",
                "Generate a git commit message for staged changes",
                r#"You are an expert at writing clear, concise git commit messages.

Analyze the staged changes and generate a commit message following these guidelines:
1. Use conventional commit format (type: description)
2. Types: feat, fix, docs, style, refactor, test, chore
3. Keep the first line under 72 characters
4. Add a blank line before any detailed explanation
5. Reference any relevant issue numbers

Steps:
1. Run `git diff --cached` to see staged changes
2. Analyze the changes and determine the commit type
3. Write an appropriate commit message"#,
            ),
            Ability::builtin(
                "review",
                "Review code changes for quality and issues",
                r#"You are an expert code reviewer. Analyze the provided code changes and provide feedback on:

1. **Correctness**: Does the code do what it's supposed to do?
2. **Code Quality**: Is the code readable, maintainable, and follows best practices?
3. **Performance**: Are there any performance concerns?
4. **Security**: Are there any security vulnerabilities?
5. **Testing**: Is there adequate test coverage?

Provide specific, actionable feedback with line references where applicable."#,
            ),
            Ability::builtin(
                "explain",
                "Explain code in detail",
                r#"You are an expert at explaining code. For the provided code:

1. Give a high-level overview of what the code does
2. Explain the key components and their interactions
3. Highlight any important patterns or techniques used
4. Point out any potential issues or improvements

Use clear, simple language suitable for developers of all skill levels."#,
            ),
            Ability::builtin(
                "test",
                "Generate tests for code",
                r#"You are an expert at writing tests. For the provided code:

1. Analyze the code to understand its functionality
2. Identify edge cases and boundary conditions
3. Generate comprehensive test cases
4. Use appropriate testing frameworks and patterns
5. Include both positive and negative test cases

Ensure tests are clear, maintainable, and provide good coverage."#,
            ),
            Ability::builtin(
                "refactor",
                "Suggest code refactoring improvements",
                r#"You are an expert at refactoring code. Analyze the provided code and suggest improvements:

1. **Code Smells**: Identify any code smells or anti-patterns
2. **Design Patterns**: Suggest applicable design patterns
3. **DRY Principle**: Find and eliminate duplication
4. **SOLID Principles**: Ensure adherence to SOLID principles
5. **Readability**: Improve naming, structure, and organization

Provide specific refactoring suggestions with before/after examples."#,
            ),
        ];

        for ability in builtin_abilities {
            self.abilities.insert(ability.name.clone(), ability);
        }
    }

    fn load_abilities_from_dir(&mut self, dir: &PathBuf, source: AbilitySource) -> io::Result<()> {
        if !dir.exists() {
            return Ok(());
        }

        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                let ability_file = path.join("ABILITY.md");
                if ability_file.exists() {
                    if let Ok(ability) = self.load_ability_from_file(&ability_file, source.clone()) {
                        self.abilities.insert(ability.name.clone(), ability);
                    }
                }
            } else if path.extension().map(|e| e == "md").unwrap_or(false) {
                if let Ok(ability) = self.load_ability_from_file(&path, source.clone()) {
                    self.abilities.insert(ability.name.clone(), ability);
                }
            }
        }

        Ok(())
    }

    fn load_ability_from_file(&self, path: &PathBuf, source: AbilitySource) -> io::Result<Ability> {
        let content = fs::read_to_string(path)?;

        let (frontmatter, body) = if content.starts_with("---") {
            if let Some(end) = content[3..].find("---") {
                let fm = &content[3..end + 3];
                let body = &content[end + 6..];
                (Some(fm), body)
            } else {
                (None, content.as_str())
            }
        } else {
            (None, content.as_str())
        };

        let mut ability = Ability::new("unknown", "No description");

        if let Some(fm) = frontmatter {
            for line in fm.lines() {
                if let Some((key, value)) = line.split_once(':') {
                    let key = key.trim();
                    let value = value.trim().trim_matches('"');

                    match key {
                        "name" => ability.name = value.to_string(),
                        "description" => ability.description = value.to_string(),
                        "when_to_use" => ability.when_to_use = Some(value.to_string()),
                        "argument_hint" => ability.argument_hint = Some(value.to_string()),
                        "allowed_tools" => {
                            ability.allowed_tools = value
                                .split(',')
                                .map(|s| s.trim().to_string())
                                .collect();
                        }
                        "model" => ability.model = Some(value.to_string()),
                        _ => {}
                    }
                }
            }
        }

        ability.content = body.to_string();
        ability.source = source;

        if ability.name == "unknown" {
            if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
                ability.name = stem.to_string();
            }
        }

        Ok(ability)
    }

    pub fn add(&mut self, ability: Ability) {
        self.abilities.insert(ability.name.clone(), ability);
    }

    pub fn remove(&mut self, name: &str) {
        self.abilities.remove(name);
    }

    pub fn get(&self, name: &str) -> Option<&Ability> {
        self.abilities.get(name)
    }

    pub fn list(&self) -> Vec<&Ability> {
        self.abilities.values().collect()
    }

    pub fn list_by_source(&self, source: AbilitySource) -> Vec<&Ability> {
        self.abilities.values().filter(|s| s.source == source).collect()
    }

    pub fn search(&self, query: &str) -> Vec<&Ability> {
        let query_lower = query.to_lowercase();

        self.abilities
            .values()
            .filter(|s| {
                s.name.to_lowercase().contains(&query_lower)
                    || s.description.to_lowercase().contains(&query_lower)
            })
            .collect()
    }

    pub fn get_ability_prompt(&self, name: &str, args: &str) -> Option<String> {
        self.abilities.get(name).map(|s| s.get_prompt(args))
    }

    pub fn get_ability_count(&self) -> usize {
        self.abilities.len()
    }

    pub fn save_ability(&self, ability: &Ability) -> io::Result<()> {
        let dir = match ability.source {
            AbilitySource::User => self.ability_dir.clone(),
            AbilitySource::Project => self.project_ability_dir.clone(),
            AbilitySource::Builtin => return Ok(()),
            AbilitySource::Plugin(_) => return Ok(()),
        };

        fs::create_dir_all(&dir)?;

        let content = format!(
            "---\nname: {}\ndescription: {}\nwhen_to_use: {}\nargument_hint: {}\nallowed_tools: {}\n---\n\n{}",
            ability.name,
            ability.description,
            ability.when_to_use.as_deref().unwrap_or(""),
            ability.argument_hint.as_deref().unwrap_or(""),
            ability.allowed_tools.join(", "),
            ability.content
        );

        let path = dir.join(format!("{}.md", ability.name));
        fs::write(path, content)?;

        Ok(())
    }
}

impl Default for AbilityManager {
    fn default() -> Self {
        Self::new()
    }
}
