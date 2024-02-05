use std::collections::HashMap;

use crate::structs::symbol_type::*;

type SymbolMap = HashMap<String, Symbol>;

#[derive(Debug, Clone)]
pub struct Symbol {
    pub pos: usize,
    pub is_global: bool,
    pub symbol_class: SymbolClass,
    pub symbol_type: SymbolType,
}

#[derive(Debug, Clone)]
pub enum SymbolClass {
    Function { parameters: Vec<SymbolType> },
    Parameter,
    Variable,
}

#[derive(Debug)]
pub struct SymbolTable {
    function_map: SymbolMap,
    stack_pointer: usize,
    global_vars: usize,

    scopes: Vec<SymbolMap>,
}

impl SymbolTable {
    pub fn new() -> Self {
        SymbolTable {
            scopes: vec![HashMap::new()],
            ..SymbolTable::default()
        }
    }

    pub fn insert_function(&mut self, name: String, return_type: SymbolType) -> Result<Symbol, String> {
        let new_sym_class = SymbolClass::Function{ parameters: vec![] };
        self.insert_symbol(name, new_sym_class, return_type)
    }

    pub fn insert_variable(&mut self, name: String, var_type: SymbolType) -> Result<Symbol, String> {
        let new_sym_class = SymbolClass::Variable;
        self.insert_symbol(name, new_sym_class, var_type)
    }

    pub fn insert_parameter(&mut self, name: String, param_type: SymbolType) -> Result<Symbol, String> {
        let new_sym_class = SymbolClass::Parameter;
        self.insert_symbol(name, new_sym_class, param_type)
    }

    fn insert_symbol(&mut self, name: String, sym_class: SymbolClass, sym_type: SymbolType) -> Result<Symbol, String> {
        let mut new_sym = Symbol{
            pos: 0,
            is_global: false,
            symbol_class: sym_class.clone(),
            symbol_type: sym_type.clone(),
        };

        match new_sym.symbol_class {
            SymbolClass::Function { .. } => {
                self.function_map.insert(name, new_sym.clone());  
                Ok(new_sym)
            }
            SymbolClass::Parameter |
            SymbolClass::Variable => {

                let is_scope_global = self.scopes.len() == 1;

                let stack_pos = if is_scope_global {
                    self.global_vars += 1;
                    self.stack_pointer
                }else{
        //            println!("Stackpointer {} global Vars: {}", self.stack_pointer, self.global_vars);
                    self.stack_pointer - self.global_vars
                };

                new_sym.pos = stack_pos;
                new_sym.is_global = is_scope_global;

                let current_scope = self.scopes.last_mut().unwrap();

                if current_scope.contains_key(&name) {
                    Err(format!("{} has been defined twice in the current scope ({})", name, self.scopes.len()))
                }else{
                    current_scope.insert(name.clone(), new_sym.clone());
                    self.stack_pointer += 1;
                    Ok(new_sym)
                }
            }
        }
    }

    pub fn append_parameter_typs_to_function_by_name(&mut self, name: String, param_types: &mut Vec<SymbolType>) {
        if ! self.function_map.contains_key(&name) {
            panic!("Eror: Function with name {:?} does not exists.", name);
        }

        let func_ref = &mut self.function_map.get_mut(&name).unwrap().symbol_class;
        if let SymbolClass::Function { ref mut parameters } = func_ref {
            parameters.append(param_types);
        }
    }

    pub fn get(&self, name: &str) -> Option<&Symbol> {
        if let Some(symbol) = self.function_map.get(name) {
            return Some(symbol);
        }else{
            for map in self.scopes.iter().rev() {
                if let Some(symbol) = map.get(name) {
                    return Some(symbol);
                }
            }
        }
        None
    }

    pub fn enter_scope(&mut self) {
        self.scopes.push(HashMap::new());
    }

    pub fn leave_scope(&mut self) {
        let symbols = self.scopes.pop().unwrap();
        self.stack_pointer -= symbols.len();
    }

    #[allow(dead_code)]
    pub fn debug_print(&self) {
        println!("DEBUG DUMP of Symbol Table:");
        println!("Declaration Scope: {:#?}", self.function_map);
        println!("Symbols in Function Stack: {:#?}", self.scopes);
    }
}

impl Default for SymbolTable {
    fn default() -> Self {
        SymbolTable {
            function_map: HashMap::new(),
            stack_pointer: 0,
            global_vars: 0, 

            scopes: vec![],
        }
    }
}