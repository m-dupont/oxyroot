#[derive(Debug, Clone)]
pub struct BranchName {
    parent: Option<Box<BranchName>>,
    name: Option<String>,
    prefix_branch: Option<String>,
}

impl Default for BranchName {
    fn default() -> Self {
        BranchName::new()
    }
}

impl BranchName {
    pub fn with_prefix<S: AsRef<str>>(mut self, prefix: S) -> Self {
        self.prefix_branch = Some(prefix.as_ref().to_string());
        self
    }

    pub fn with_name<S: AsRef<str>>(mut self, name: S) -> Self {
        self.name = Some(name.as_ref().to_string());
        self
    }

    pub fn make_absolute<S: AsRef<str>>(&self, name: S) -> Self {
        let mut b = BranchName::new();
        b.name = Some(name.as_ref().to_string());
        b.prefix_branch = None;
        b
    }

    pub fn new() -> Self {
        BranchName {
            parent: None,
            name: None,
            prefix_branch: None,
        }
    }

    pub fn make_child<S: AsRef<str>>(&self, name: S) -> Self {
        let mut b = BranchName::new();

        match &self.name {
            None => {}
            Some(_name) => b.parent = Box::new(self.clone()).into(),
        };
        b.name = Some(name.as_ref().to_string());
        b
    }

    pub fn final_name(&self) -> String {
        let mut final_branch_name = String::new();

        if let Some(parent) = &self.parent {
            final_branch_name.push_str(&parent.final_name());
            final_branch_name.push('.');
        }

        if let Some(prefix) = &self.prefix_branch {
            final_branch_name.push_str(prefix);
        }

        match &self.name {
            None => {}
            Some(s) => {
                final_branch_name.push_str(s);
            }
        }
        final_branch_name
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_branch_name() {
        let branch = BranchName::new().with_name("name").with_prefix("prefix");
        assert_eq!(branch.final_name(), "prefixname");

        let branch = BranchName::new().with_prefix("prefix");
        assert_eq!(branch.final_name(), "prefix");

        let branch = BranchName::new().with_name("name");
        assert_eq!(branch.final_name(), "name");

        let branch = branch.make_child("fils");
        assert_eq!(branch.final_name(), "name.fils");

        let branch = BranchName::new()
            .with_name("name")
            .with_prefix("prefix")
            .make_child("fils");
        assert_eq!(branch.final_name(), "prefixname.fils");

        let branch = BranchName::new().make_child("fils");
        assert_eq!(branch.final_name(), "fils");

        let branch = BranchName::new()
            .with_name("name")
            .with_prefix("prefix")
            .make_child("fils");
        assert_eq!(branch.final_name(), "prefixname.fils");

        let branch_a = branch.make_absolute("fils");
        assert_eq!(branch_a.final_name(), "fils");

        let branch_a = branch.make_absolute("fils").with_prefix("prefix.");
        assert_eq!(branch_a.final_name(), "prefix.fils");

        let branch_a = branch.with_prefix("prefix.").make_absolute("fils");
        assert_eq!(branch_a.final_name(), "fils");
    }
}
