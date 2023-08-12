use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq, Clone)]
pub enum SymbolScope {
    Global,
    Local,
    Builtin,
    Free,
    Function,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Symbol {
    pub name: String,
    pub scope: SymbolScope,
    pub index: usize,
}

#[derive(Default, Clone, Debug, PartialEq)]
pub struct SymbolTable {
    pub outer: Option<Rc<RefCell<Self>>>,

    store: HashMap<String, Symbol>,
    pub num_definitions: usize,

    pub free_symbols: Vec<Symbol>,
}

impl SymbolTable {
    pub fn new() -> Self {
        Self {
            outer: None,

            store: HashMap::new(),
            num_definitions: 0,

            free_symbols: vec![],
        }
    }

    pub fn new_enclosed(enclosing: Rc<RefCell<Self>>) -> Self {
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

    pub fn define_builtin(&mut self, index: usize, name: String) -> Symbol {
        let sym = Symbol {
            name: name.clone(),
            scope: SymbolScope::Builtin,
            index,
        };
        self.store.insert(name, sym.clone());
        sym
    }

    pub fn resolve(&mut self, name: &str) -> Option<Symbol> {
        if let Some(obj) = self.store.get(name) {
            return Some(obj.clone());
        }

        if let Some(outer) = self.outer.clone() {
            //TODO: Change this
            match outer.borrow_mut().resolve(name) {
                Some(sym) => {
                    if sym.scope == SymbolScope::Global || sym.scope == SymbolScope::Builtin {
                        Some(sym)
                    } else {
                        let free = self.define_free(sym);
                        Some(free)
                    }
                }
                None => None,
            }
        } else {
            None // If there's no outer or if it is None, return None
        }
    }
    fn define_free(&mut self, original: Symbol) -> Symbol {
        let name = original.name.clone();
        let sym = Symbol {
            name,
            scope: SymbolScope::Free,
            index: self.free_symbols.len(),
        };
        self.free_symbols.push(original);
        self.store.insert(sym.name.clone(), sym.clone());
        sym
    }

    pub fn define_function_name(&mut self, name: String) -> Symbol {
        let symbol = Symbol {
            name,
            scope: SymbolScope::Function,
            index: 0,
        };
        self.store.insert(symbol.name.clone(), symbol.clone());
        symbol
    }

    pub fn has_outer(&self) -> bool{
        self.outer.is_some()
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

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));

        let result = first_local.define("c".to_string());
        assert_eq!(result, expected["c"]);

        let result = first_local.define("d".to_string());
        assert_eq!(result, expected["d"]);

        let mut second_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local)));

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

        let mut local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));
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

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global)));
        first_local.define("c".to_string());
        first_local.define("d".to_string());

        let mut second_local =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local.clone())));
        second_local.define("e".to_string());
        second_local.define("f".to_string());

        let expected = vec![
            (
                &mut first_local,
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
                &mut second_local,
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

    #[test]
    fn test_resolve_builtins() {
        let mut global = SymbolTable::default();

        let expected = vec![
            Symbol {
                name: "a".to_string(),
                scope: SymbolScope::Builtin,
                index: 0,
            },
            Symbol {
                name: "c".to_string(),
                scope: SymbolScope::Builtin,
                index: 1,
            },
            Symbol {
                name: "e".to_string(),
                scope: SymbolScope::Builtin,
                index: 2,
            },
            Symbol {
                name: "f".to_string(),
                scope: SymbolScope::Builtin,
                index: 3,
            },
        ];

        for (i, sym) in expected.iter().enumerate() {
            global.define_builtin(i, sym.name.clone());
        }

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global.clone())));
        let mut second_local =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local.clone())));

        for table in vec![&mut global, &mut first_local, &mut second_local] {
            for sym in expected.clone() {
                let result = table.resolve(&sym.name);

                assert!(result.is_some());
                assert_eq!(result.unwrap(), sym);
            }
        }
    }

    #[test]
    fn test_resolve_free() {
        let mut global = SymbolTable::new();
        global.define("a".to_string());
        global.define("b".to_string());

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global.clone())));
        first_local.define("c".to_string());
        first_local.define("d".to_string());

        let mut second_local =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local.clone())));
        second_local.define("e".to_string());
        second_local.define("f".to_string());

        let tests = vec![
            (
                &mut first_local,
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
                vec![],
            ),
            (
                &mut second_local,
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
                        scope: SymbolScope::Free,
                        index: 0,
                    },
                    Symbol {
                        name: "d".to_string(),
                        scope: SymbolScope::Free,
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
                vec![
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
        ];

        for test in tests {
            let (local, expected_symbols, expected_free_symbols) = test;
            for sym in expected_symbols {
                let result = local.resolve(&sym.name);

                assert!(result.is_some());
                assert_eq!(result.unwrap(), sym);
            }

            assert_eq!(local.free_symbols.len(), expected_free_symbols.len());

            for (i, sym) in expected_free_symbols.iter().enumerate() {
                let result = local.free_symbols.get(i).unwrap();

                assert_eq!(result, sym);
            }
        }
    }

    #[test]
    fn tests_unresovable_free() {
        let mut global = SymbolTable::new();
        global.define("a".to_string());

        let mut first_local = SymbolTable::new_enclosed(Rc::new(RefCell::new(global.clone())));
        first_local.define("c".to_string());

        let mut second_local =
            SymbolTable::new_enclosed(Rc::new(RefCell::new(first_local.clone())));
        second_local.define("e".to_string());
        second_local.define("f".to_string());

        let expected = vec![
            Symbol {
                name: "a".to_string(),
                scope: SymbolScope::Global,
                index: 0,
            },
            Symbol {
                name: "c".to_string(),
                scope: SymbolScope::Free,
                index: 0,
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
        ];

        for sym in expected {
            let result = second_local.resolve(&sym.name);

            assert!(result.is_some());
            assert_eq!(result.unwrap(), sym);
        }

        let expect_unresolvable = vec!["b".to_string(), "d".to_string()];

        for sym in expect_unresolvable {
            let result = second_local.resolve(&sym);

            assert!(result.is_none());
        }
    }

    #[test]
    fn test_defini_and_resolve_function_name() {
        let mut global = SymbolTable::new();
        global.define_function_name("a".to_string());

        let expected = Symbol {
            name: "a".to_string(),
            scope: SymbolScope::Function,
            index: 0,
        };

        let result = global.resolve(expected.name.as_str());

        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }

    #[test]
    fn test_shadowing_function_name() {
        let mut global = SymbolTable::new();
        global.define_function_name("a".to_string());
        global.define("a".to_string());

        let expected = Symbol {
            name: "a".to_string(),
            scope: SymbolScope::Global,
            index: 0,
        };

        let result = global.resolve(expected.name.as_str());

        assert!(result.is_some());
        assert_eq!(result.unwrap(), expected);
    }
}
