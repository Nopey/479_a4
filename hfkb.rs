///! Implements a Horn Form Knowledge base, with clauses specified by strings.
	
use std::collections::{HashMap,HashSet};

/// A horn form knowledge base
pub struct HornFormKb {
	/// A mapping from symbol name to VarHandle
	symbols: HashMap<String, VarHandle>,
	/// Indexed by VarHandle
	vars: Vec<Var>,
}

impl HornFormKb {
	pub fn new() -> Self {
		HornFormKb {
			symbols: HashMap::new(),
			vars: vec![],
		}
	}

	/// Adds a clause to the knowledge base, and also applies
	/// inference, to simplify the knowledge base.
	pub fn tell(&mut self, expr: &str, symbol: &str) {
		// vars[u] is implied by this horn clause
		let u = self.parse_symbol(symbol);

		// If it's already true, no more work is necessary.
		if self.vars[u].is_true() {
			return;
		}

		let clause_handle = (u, self.vars[u].clauses.len());
		self.vars[u].clauses.push(HornClause::new());
		for symbol in expr.split('^') {
			if symbol.is_empty() {
				continue;
			}

			// u <- a1^a2^a3
			// a is one of the variables that must be true for u to be implied.
			let a = self.parse_symbol(symbol);

			// if a is already true, we can simplify by removing 'a' from the expression
			let var_a = &mut self.vars[a];
			if var_a.is_true() {
				continue;
			}

			// Add this clause's dependancy to 'a'
			var_a.dependants.push(clause_handle);
			// Add 'a' to this clause's dependancies
			let clause = self.vars[u].clauses.last_mut().unwrap();
			clause.depends.insert(a);
		}

		// if no 'depends' remain, this var is now true.
		if self.vars[u].clauses.last().unwrap().depends.is_empty() {
			// var handles that have newly become true
			// (order is irrelevant, so using a stack)
			let mut work_queue = vec![u];
			while let Some(u) = work_queue.pop() {
				// replace 'clauses' with a single factual HornClause
				self.vars[u].clauses.clear();
				self.vars[u].clauses.push(HornClause::new());

				// go through dependant clauses and remove 'u'
				let dependants = std::mem::take(&mut self.vars[u].dependants);
				for &(d,c) in dependants.iter() {
					self.vars[d].clauses[c].depends.remove(&u);
					if self.vars[d].clauses[c].is_fact() {
						work_queue.push(d);
					}
				}
			}
		}
	}

	/// Get a symbol's Variable Handle, creating the
	/// variable if this symbol hasn't been seen before.
	fn parse_symbol(&mut self, symbol: &str) -> VarHandle {
		let HornFormKb{ symbols, vars } = self;
		*symbols.entry(symbol.to_owned())
			.or_insert_with(||{
				vars.push(Var::new());
				vars.len() - 1
		})
	}

	/// Is the variable referred to the symbol query entailed in the knowledge base?
	pub fn ask(&self, query: &str) -> bool {
		let vh =
			if let Some(&vh) = self.symbols.get(query) { vh }
			// symbol not in kb, so query is false.
			else { return false; };

		self.vars[vh].is_true()
	}
}


/// An index into the knowledge base's vars Vec
type VarHandle = usize;
/// An index into the clauses Vec on the variable referred to by VarHandle.
type ClauseHandle = (VarHandle, usize);
/// A variable in a hfkb, containing zero or more clauses.
///
/// Also contains some metadata for ::tell's simplification.
struct Var {
	/// This Variable is true if at least one of the clauses are.
	clauses: Vec<HornClause>,
	/// Dependant Clauses
	///
	/// Used to remove dependancy when this var becomes true.
	dependants: Vec<ClauseHandle>,
}

impl Var {
	fn new() -> Self {
		Var {
			clauses: vec![],
			dependants: vec![],
		}
	}
	/// Is this variable entailed in the knowledge base?
    fn is_true(&self) -> bool {
		// Invariant: Any var that is true has a single clause, which is a fact.
		// This is maintained in HornFormKb::tell above
		self.clauses.get(0).map(HornClause::is_fact) == Some(true)
	}
}


/// A horn clause, a set of variables that are AND'd together
///
/// This struct does not know what variable is implied by the clause.
struct HornClause {
	/// This Horn Clause is true if all of the variable terms ("depended on variables") are.
	depends: HashSet<VarHandle>,
}

impl HornClause {
	fn new() -> Self {
		HornClause {
			depends: HashSet::new(),
		}
	}

	/// Is this clause a fact or, equivalently, does this clause have no variable terms?
	fn is_fact(&self) -> bool {
		self.depends.is_empty()
	}
}
