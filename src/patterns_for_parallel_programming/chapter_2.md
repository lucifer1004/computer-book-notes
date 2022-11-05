# Background and Jargon of Parallel Computing

<!-- toc -->

## 2.1 Concurrency in Parallel Programs versus Operating Systems

## 2.2 Parallel Architectures: A Brief Introduction

### 2.2.1 Flynn's Taxonomy

Four possibilities:

- SISD (Single Instruction, Single Data)
- SIMD (Single Instruction, Multiple Data)
- MISD: mentioned for completeness
- MIMD

### 2.2.2 A Further Breakdown of MIMD

- Shared memory
  - SMP (Symmetric Multiprocessors)
  - NUMA (Non-Uniform Memory Access)
- Distributed memory
  - MPP (Massively Parallel Processors)
  - Clusters
- Hybrid

### 2.2.3 Summary

## 2.3 Parallel Programming Environments

Before: a long, long list...

Now:

- OpenMP: a set of extensions to sequential programming languages
- MPI: a library of routines to be called from programs written in a sequential programming language
- Hybrid: OpenMP for each node and MPI between nodes
- Built-in: Languages with built-in features to support parallel programming

## 2.4 The Jargon of Parallel Computing

- Task
- Unit of execution (UE)
- Processing element (PE)
- Load balancing
- Synchronization
- Synchronous versus asynchronous
- Race conditions
- Deadlocks

## 2.5 A Quantitative Look at Parallel Computation

A few formulae for measuring the efficiency.

## 2.6 Communication

### 2.6.1 Latency and Bandwidth

Fixed cost (latency) plus cost proportional to the amount of data (data / bandwidth).

### 2.6.2 Overlapping Communication and Computation and Latency Hiding

Continue computation instead of waiting for the message idly.

## 2.7 Summary
