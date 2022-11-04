# Chapter 1 Propositional Logic

<!-- toc -->

## 1.1 Syntax

- not: \\(\neg F\\)
- and: \\(F_1\land F_2\\)
- or: \\(F_1\lor F_2\\)
- implication: \\(F_1\implies F_2\\)
- if and only if: \\(F_1\iff F_2\\)

Precedence from highest to lowest: \\(\neg\\), \\(\land\\), \\(\lor\\), \\(\implies\\), \\(\iff\\)

Associativity of binary operators: \\(\land\\) and \\(\lor\\) are left-associative, \\(\implies\\) and \\(\iff\\) are right-associative.

## 1.2 Semantics

An **interpretation**:

\\[I: P\mapsto\mathrm{true}, Q\mapsto\mathrm{false},\dots\\]

A **truth table**:

| \\(F\\) | \\(\neg F\\) |
| ------- | ------------ |
| 0       | 1            |
| 1       | 0            |

Given that all symbols used are included in the interpretation, we can evaluate an arbitrary expression.

**Constants**:

\\[\forall I, I\vDash\top, I\nvDash\bot\\]

## 1.3 Satisfiability and Validity

### 1.3.1 Truth Tables

### 1.3.2 Semantic Arguments

## 1.4 Equivalence and Implication

## 1.5 Substitution

## 1.6 Normal Forms

## 1.7 Decision Procedures for Satisfiability

### 1.7.1 Simple Decision Procedures

### 1.7.2 Reconsidering the Truth-Table Method

### 1.7.3 Conversion to an Equisatisfiable Formula in CNF

### 1.7.4 The Resolution Procedure

### 1.7.5 DPLL

## 1.8 Summary

## Exercises
