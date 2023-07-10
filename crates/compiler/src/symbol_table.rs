use std::collections::HashMap;

#[derive(Debug, PartialEq, Clone)]
enum SymbolScope {
    Global,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    name: String,
    scope: SymbolScope,
    pub index: usize,
}

#[derive(Default)]
pub struct SymbolTable {
    store: HashMap<String, Symbol>,
    num_definitions: usize,
}

impl SymbolTable {
    pub fn define(&mut self, name: String) -> Symbol {
        let symbol = Symbol {
            name: name.clone(),
            scope: SymbolScope::Global,
            index: self.num_definitions,
        };

        self.store.insert(name, symbol.clone());
        self.num_definitions += 1;

        symbol
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        self.store.get(name).cloned()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_define() {
        let mut global = SymbolTable::default();
        let mut expected = HashMap::new();

        expected.insert(
            "a".to_string(),
            Symbol {
                name: "a".to_string(),
                scope: SymbolScope::Global,
                index: 0,
            },
        );

        expected.insert(
            "b".to_string(),
            Symbol {
                name: "b".to_string(),
                scope: SymbolScope::Global,
                index: 1,
            },
        );

        let result = global.define("a".to_string());
        assert_eq!(result, expected["a"]);

        let result = global.define("b".to_string());
        assert_eq!(result, expected["b"]);
    }

    #[test]
    fn test_resolve_global() {
        let mut global = SymbolTable::default();
        global.define("a".to_string());
        global.define("b".to_string());

        let expected = Some(Symbol {
            name: "a".to_string(),
            scope: SymbolScope::Global,
            index: 0,
        });

        let result = global.resolve("a");
        assert_eq!(result, expected);

        let expected = Some(Symbol {
            name: "b".to_string(),
            scope: SymbolScope::Global,
            index: 1,
        });

        let result = global.resolve("b");
        assert_eq!(result, expected);
    }
}
