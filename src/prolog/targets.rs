use prolog_parser::ast::*;

use crate::prolog::clause_types::*;
use crate::prolog::forms::*;
use crate::prolog::instructions::*;
use crate::prolog::iterators::*;

pub trait CompilationTarget<'a> {
    type Iterator: Iterator<Item = TermRef<'a>>;

    fn iter(_: &'a Term) -> Self::Iterator;

    fn to_constant(_: Level, _: Constant, _: RegType) -> Self;
    fn to_list(_: Level, _: RegType) -> Self;
    fn to_structure(_: ClauseType, _: usize, _: RegType) -> Self;

    fn to_void(_: usize) -> Self;
    fn is_void_instr(&self) -> bool;

    fn incr_void_instr(&mut self);

    fn constant_subterm(_: Constant) -> Self;

    fn argument_to_variable(_: RegType, _: usize) -> Self;
    fn argument_to_value(_: RegType, _: usize) -> Self;

    fn move_to_register(_: RegType, _: usize) -> Self;

    fn subterm_to_variable(_: RegType) -> Self;
    fn subterm_to_value(_: RegType) -> Self;

    fn clause_arg_to_instr(_: RegType) -> Self;
}

impl<'a> CompilationTarget<'a> for FactInstruction {
    type Iterator = FactIterator<'a>;

    fn iter(term: &'a Term) -> Self::Iterator {
        breadth_first_iter(term, false) // do not iterate over the root clause if one exists.
    }

    fn to_constant(lvl: Level, constant: Constant, reg: RegType) -> Self {
        FactInstruction::GetConstant(lvl, constant, reg)
    }

    fn to_structure(ct: ClauseType, arity: usize, reg: RegType) -> Self {
        FactInstruction::GetStructure(ct, arity, reg)
    }

    fn to_list(lvl: Level, reg: RegType) -> Self {
        FactInstruction::GetList(lvl, reg)
    }

    fn to_void(subterms: usize) -> Self {
        FactInstruction::UnifyVoid(subterms)
    }

    fn is_void_instr(&self) -> bool {
        match self {
            &FactInstruction::UnifyVoid(_) => true,
            _ => false,
        }
    }

    fn incr_void_instr(&mut self) {
        match self {
            &mut FactInstruction::UnifyVoid(ref mut incr) => *incr += 1,
            _ => {}
        }
    }

    fn constant_subterm(constant: Constant) -> Self {
        FactInstruction::UnifyConstant(constant)
    }

    fn argument_to_variable(arg: RegType, val: usize) -> Self {
        FactInstruction::GetVariable(arg, val)
    }

    fn move_to_register(arg: RegType, val: usize) -> Self {
        FactInstruction::GetVariable(arg, val)
    }

    fn argument_to_value(arg: RegType, val: usize) -> Self {
        FactInstruction::GetValue(arg, val)
    }

    fn subterm_to_variable(val: RegType) -> Self {
        FactInstruction::UnifyVariable(val)
    }

    fn subterm_to_value(val: RegType) -> Self {
        FactInstruction::UnifyValue(val)
    }

    fn clause_arg_to_instr(val: RegType) -> Self {
        FactInstruction::UnifyVariable(val)
    }
}

impl<'a> CompilationTarget<'a> for QueryInstruction {
    type Iterator = QueryIterator<'a>;

    fn iter(term: &'a Term) -> Self::Iterator {
        post_order_iter(term)
    }

    fn to_structure(ct: ClauseType, arity: usize, r: RegType) -> Self {
        QueryInstruction::PutStructure(ct, arity, r)
    }

    fn to_constant(lvl: Level, constant: Constant, reg: RegType) -> Self {
        QueryInstruction::PutConstant(lvl, constant, reg)
    }

    fn to_list(lvl: Level, reg: RegType) -> Self {
        QueryInstruction::PutList(lvl, reg)
    }

    fn to_void(subterms: usize) -> Self {
        QueryInstruction::SetVoid(subterms)
    }

    fn is_void_instr(&self) -> bool {
        match self {
            &QueryInstruction::SetVoid(_) => true,
            _ => false,
        }
    }

    fn incr_void_instr(&mut self) {
        match self {
            &mut QueryInstruction::SetVoid(ref mut incr) => *incr += 1,
            _ => {}
        }
    }

    fn constant_subterm(constant: Constant) -> Self {
        QueryInstruction::SetConstant(constant)
    }

    fn argument_to_variable(arg: RegType, val: usize) -> Self {
        QueryInstruction::PutVariable(arg, val)
    }

    fn move_to_register(arg: RegType, val: usize) -> Self {
        QueryInstruction::GetVariable(arg, val)
    }

    fn argument_to_value(arg: RegType, val: usize) -> Self {
        QueryInstruction::PutValue(arg, val)
    }

    fn subterm_to_variable(val: RegType) -> Self {
        QueryInstruction::SetVariable(val)
    }

    fn subterm_to_value(val: RegType) -> Self {
        QueryInstruction::SetValue(val)
    }

    fn clause_arg_to_instr(val: RegType) -> Self {
        QueryInstruction::SetValue(val)
    }
}
