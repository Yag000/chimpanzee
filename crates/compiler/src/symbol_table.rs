use std::{collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    Global,
    Local,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct SymbolTable {
    pub outer: Option<Rc<Self>>,

    store: HashMap<String, Symbol>,
    pub num_definitions: usize,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            outer: None,

            store: HashMap::new(),
            num_definitions: 0,
        }
    }

    pub fn new_enclosed(enclosing: Rc<Self>) -> Self {
        let mut new = Self::new();
        new.outer = Some(enclosing);
        new
    }

    pub fn define(&mut self, name: String) -> Symbol {
        let scope = match self.outer {
            Some(_) => SymbolScope::Local,
            None => SymbolScope::Global,
        };

        let symbol = Symbol {
            name: name.clone(),
            scope,
            index: self.num_definitions,
        };

        self.store.insert(name, symbol.clone());
        self.num_definitions += 1;

        symbol
    }

    pub fn resolve(&self, name: &str) -> Option<Symbol> {
        match self.store.get(name) {
            Some(obj) => Some(obj.clone()),
            None => match &self.outer {
                Some(outer) => outer.resolve(name),
                None => None,
            },
        }
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

        expected.insert(
            "c".to_string(),
            Symbol {
                name: "c".to_string(),
                scope: SymbolScope::Local,
                index: 0,
            },
        );

        expected.insert(
            "d".to_string(),
            Symbol {
                name: "d".to_string(),
                scope: SymbolScope::Local,
                index: 1,
            },
        );

        expected.insert(
            "e".to_string(),
            Symbol {
                name: "e".to_string(),
                scope: SymbolScope::Local,
                index: 0,
            },
        );

        expected.insert(
            "f".to_string(),
            Symbol {
                name: "f".to_string(),
                scope: SymbolScope::Local,
                index: 1,
            },
        );

        let result = global.define("a".to_string());
        assert_eq!(result, expected["a"]);

        let result = global.define("b".to_string());
        assert_eq!(result, expected["b"]);

        let mut first_local = SymbolTable::new_enclosed(Rc::new(global));

        let result = first_local.define("c".to_string());
        assert_eq!(result, expected["c"]);

        let result = first_local.define("d".to_string());
        assert_eq!(result, expected["d"]);

        let mut second_local = SymbolTable::new_enclosed(Rc::new(first_local));

        let result = second_local.define("e".to_string());
        assert_eq!(result, expected["e"]);

        let result = second_local.define("f".to_string());
        assert_eq!(result, expected["f"]);
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

    #[test]
    fn test_resolve_local() {
        let mut global = SymbolTable::default();
        global.define("a".to_string());
        global.define("b".to_string());

        let mut local = SymbolTable::new_enclosed(Rc::new(global));
        local.define("c".to_string());
        local.define("d".to_string());

        let expected = vec![
            Symbol {
                name: "a".to_string(),
                scope: SymbolScope::Global,
                index: 0,
            },
            Symbol {
                name: "b".to_string(),
                scope: SymbolScope::Global,
                index: 1,
            },
            Symbol {
                name: "c".to_string(),
                scope: SymbolScope::Local,
                index: 0,
            },
            Symbol {
                name: "d".to_string(),
                scope: SymbolScope::Local,
                index: 1,
            },
        ];

        for sym in expected {
            let result = local.resolve(&sym.name);

            assert!(result.is_some());
            assert_eq!(result.unwrap(), sym);
        }
    }

    #[test]
    fn test_resolve_nested_local() {
        let mut global = SymbolTable::default();
        global.define("a".to_string());
        global.define("b".to_string());

        let mut first_local = SymbolTable::new_enclosed(Rc::new(global));
        first_local.define("c".to_string());
        first_local.define("d".to_string());

        let mut second_local = SymbolTable::new_enclosed(Rc::new(first_local.clone()));
        second_local.define("e".to_string());
        second_local.define("f".to_string());

        let expected = vec![
            (
                first_local,
                vec![
                    Symbol {
                        name: "a".to_string(),
                        scope: SymbolScope::Global,
                        index: 0,
                    },
                    Symbol {
                        name: "b".to_string(),
                        scope: SymbolScope::Global,
                        index: 1,
                    },
                    Symbol {
                        name: "c".to_string(),
                        scope: SymbolScope::Local,
                        index: 0,
                    },
                    Symbol {
                        name: "d".to_string(),
                        scope: SymbolScope::Local,
                        index: 1,
                    },
                ],
            ),
            (
                second_local,
                vec![
                    Symbol {
                        name: "a".to_string(),
                        scope: SymbolScope::Global,
                        index: 0,
                    },
                    Symbol {
                        name: "b".to_string(),
                        scope: SymbolScope::Global,
                        index: 1,
                    },
                    Symbol {
                        name: "e".to_string(),
                        scope: SymbolScope::Local,
                        index: 0,
                    },
                    Symbol {
                        name: "f".to_string(),
                        scope: SymbolScope::Local,
                        index: 1,
                    },
                ],
            ),
        ];

        for test in expected {
            let (local, test) = test;
            for sym in test {
                let result = local.resolve(&sym.name);

                assert!(result.is_some());
                assert_eq!(result.unwrap(), sym);
            }
        }
    }
}
