# Six-Month Engineering Interview & AI Upskilling Roadmap (SDE II)

**Planning horizon:** 26 weeks (approximately six months)\
**Tracks:** Java Interview Preparation · NeetCode 250 · Low-Level Design
· High-Level Design · AI Engineering Upskilling (LLMs, RAG, Agents, MCP)\
**Intended learner:** SDE II backend engineer with professional
Java/Spring Boot experience, preparing for stronger SDE interviews while
upskilling for the agentic AI era: understanding how LLMs work, building
RAG and agent systems, evaluating them, and using AI tools effectively.\
**Recommended study capacity:** 7--8 focused hours per week outside
work. Increase only if the routine remains sustainable.

------------------------------------------------------------------------

## 1. How to use this roadmap

This document is deliberately structured so a study-planning application
can turn topics into daily tasks. Treat each topic row as a learning
unit and each subtopic as a checklist. Do not mark a topic complete
merely because a video or article was consumed.

### Completion states

  -----------------------------------------------------------------------
  State                               Meaning
  ----------------------------------- -----------------------------------
  Not started                         No meaningful work completed

  Learning                            Studying the concept and taking
                                      notes

  Practicing                          Solving exercises, coding, or
                                      designing

  Needs revision                      Previously studied but not yet
                                      independently recalled

  Interview ready                     Can explain, apply, compare
                                      trade-offs, and handle follow-up
                                      questions

  Mastered                            Can reproduce the
                                      implementation/design without notes
                                      and teach it clearly
  -----------------------------------------------------------------------

### Priority definitions

-   **High:** Frequently tested, foundational, or directly useful in the
    target role. Do not defer.
-   **Medium:** Important for depth and stronger interviews; study after
    prerequisites.
-   **Low:** Useful differentiator or niche depth. Cover after
    high-priority material.

### Time estimates

Time estimates below are **focused learning + practice time**, not
passive video duration. They are planning estimates, not deadlines. Add
time when a topic exposes a real gap.

### Daily learning-unit definition

A useful daily task should be small enough to complete in 30--90 minutes
and should have a concrete output. Examples:

-   Explain `HashMap` collision handling from memory and write a small
    collision experiment.
-   Solve one assigned NeetCode problem, then write its invariant and
    complexity.
-   Design the class model for a parking lot before watching the
    solution.
-   Draw a write-heavy feed architecture and explain the database/cache
    trade-offs.
-   Implement a document ingestion and retrieval vertical slice, then
    evaluate it against test questions.

### Important rules

1.  **DSA source of truth is the official NeetCode 250 sheet.** Do not
    insert random LeetCode problems into the DSA track. Use the exact
    problem IDs, names, topic group, and official order from the sheet.
2.  **Resume from actual progress.** The plan must import or map the
    learner's current NeetCode completion state; it must not assume
    every topic is untouched.
3.  **CodeArmy is the primary LLD video curriculum.** Attempt each
    practical design problem before watching its solution.
4.  **Do not confuse HLD and LLD.** HLD is about system boundaries, data
    flow, scaling, reliability and trade-offs. LLD is about object
    models, APIs, responsibilities, extensibility and implementation.
5.  **AI upskilling is not model research.** Focus on understanding LLMs
    well enough to build, evaluate and operate reliable software with
    existing models and APIs, not training a foundation model from
    scratch.
6.  **AI is a mentor/reviewer during learning tasks.** First attempt the
    explanation/design/code independently. Ask for hints, critique, test
    cases and review---not a complete solution by default.
7.  **A missed day does not create debt.** Re-plan unfinished work; do
    not double the next day's workload.

------------------------------------------------------------------------

# 2. Six-month execution strategy

## 2.1 Recommended weekly allocation

A realistic baseline is 7--8 hours per week. The distribution changes
slightly across phases.

  ------------------------------------------------------------------------
  Track                               Weekly average Why
  --------------------- ---------------------------- ---------------------
  DSA --- NeetCode 250                  2.5--3 hours Consistency and
                                                     pattern retention
                                                     matter more than
                                                     occasional marathons

  Java interview                       1--1.25 hours Deepen runtime,
  preparation                                        collections,
                                                     concurrency and
                                                     framework
                                                     fundamentals

  LLD --- CodeArmy                            1 hour Video + independent
                                                     implementation

  HLD                                  1--1.25 hours Build conceptual
                                                     breadth and repeated
                                                     design practice

  AI engineering upskilling             2--2.25 hours Concept, raw API
                                                     code, then framework;
                                                     one capstone project
                                                     built incrementally
  ------------------------------------------------------------------------

This is a baseline, not a rigid quota. During a capstone milestone,
borrow time from optional Java/LLD depth---not from sleep or recovery.
During an interview-heavy period, temporarily shift time toward DSA,
Java and design mocks.

## 2.2 Suggested weekly cadence

  -------------------------------------------------------------------------
  Day              Session                       Duration Expected output
  ---------------- ---------------- --------------------- -----------------
  Monday           NeetCode 250                45--60 min One problem
                                                          attempt or a
                                                          deliberate
                                                          revision

  Tuesday          Java                        45--60 min One concept
                                                          explained + one
                                                          code experiment

  Wednesday        NeetCode 250                45--60 min One problem +
                                                          pattern note

  Thursday         LLD or HLD                  45--60 min Diagram, class
                   (alternate                             model, or design
                   weekly)                                explanation

  Friday           NeetCode                    30--45 min Re-solve/review
                   review + Java                          and answer
                   recall                                 questions without
                                                          notes

  Saturday         AI upskilling +           2--2.5 hours A working project
                   design practice                        increment and/or
                                                          one design
                                                          exercise

  Sunday           Weekly review               30--45 min Revision queue,
                                                          progress update,
                                                          next-week plan
  -------------------------------------------------------------------------

**Minimum viable session:** 20 minutes. On a difficult day, do one small
task---one recall exercise, one problem review, or one code experiment.
Preserve continuity without turning study into punishment.

## 2.3 Six-month phases

  -----------------------------------------------------------------------------
  Phase                                        Weeks Primary outcome
  --------------------- ---------------------------- --------------------------
  Phase 1 ---                                   1--4 Establish Java/runtime
  Foundations and                                    fundamentals, refresh DSA
  baseline                                           patterns, OOP/SOLID, HLD
                                                     fundamentals, Python, LLM
                                                     fundamentals

  Phase 2 --- Core                              5--9 Collections/concurrency,
  interview depth                                    trees/graphs/DP progress,
                                                     design patterns,
                                                     databases/caching,
                                                     LLM APIs, JSON, tools/MCP

  Phase 3 --- Applied                         10--14 Java performance and
  engineering                                        Spring internals, advanced
                                                     DSA, LLD projects,
                                                     distributed systems,
                                                     vector DBs, RAG slice

  Phase 4 ---                                 15--19 Timed DSA,
  Production readiness                               concurrency/testing, HLD
                                                     case studies, evals,
                                                     agents, memory,
                                                     LangGraph

  Phase 5 --- Interview                       20--23 Mock interviews, design
  execution                                          trade-offs, project
                                                     hardening, weak-area
                                                     remediation

  Phase 6 ---                                 24--26 Revisions, timed mocks,
  Consolidation                                      project walkthrough,
                                                     resume stories and
                                                     application execution
  -----------------------------------------------------------------------------

**Do not wait until week 26 to apply.** Begin targeted applications and
interview practice once the fundamentals are sufficiently
refreshed---typically during phases 2--3. Use real interview feedback to
reprioritize.

------------------------------------------------------------------------

# 3. Track A --- Java Interview Preparation (Senior SDE)

## Track objective

Develop the ability to explain Java internals, make sound implementation
choices, diagnose concurrency/performance issues, and connect
language/runtime behavior to production Spring Boot services.

This is not a beginner Java course. Skip basic syntax you already use
professionally; focus on the *why*, edge cases, trade-offs, and
interview follow-ups.

**Estimated total:** 45--55 focused hours across six months, including
spaced revision.\
**Priority model:** High for language, collections, concurrency, JVM,
SQL/transactions and Spring; Medium for advanced runtime/tooling topics.

### A1. Language, object model and type system

  ----------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ----------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              OOP, object identity, equality,
                                      immutability, generics, exceptions

  Subtopics                           `==` vs `equals`; `hashCode` contract;
                                      overriding vs overloading; inheritance
                                      vs composition; abstract class vs
                                      interface; access modifiers;
                                      static/final; shallow vs deep copy;
                                      immutable objects; defensive copying;
                                      generics, wildcards, PECS, type erasure;
                                      checked vs unchecked exceptions;
                                      try-with-resources

  Learning resources                  [Dev.java --- Language
                                      basics](https://dev.java/learn/);
                                      [Dev.java --- Object-oriented
                                      programming](https://dev.java/learn/);
                                      Effective Java by Joshua Bloch (selected
                                      items on equals/hashCode, immutability,
                                      composition, generics and exceptions)

  Practical work                      Implement an immutable `Money`/`Trade`
                                      value object; demonstrate equality/hash
                                      contract failures in a `HashSet`; write
                                      generic utility methods and bounded
                                      wildcards

  Completion criteria                 Explain equality/hash contract; identify
                                      mutability hazards; choose
                                      interface/abstract class/composition
                                      appropriately; explain type erasure and
                                      PECS with examples

  Dependencies                        None
  ----------------------------------------------------------------------------

### A2. Collections framework and internals

  -----------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Collection interfaces, List, Set, Map, Queue,
                                      iterators

  Subtopics                           `ArrayList` vs `LinkedList`; `HashMap` internals
                                      (hash spreading, buckets, collision handling, tree
                                      bins, resize/load factor); `HashSet` implementation;
                                      `LinkedHashMap`; `TreeMap`/`TreeSet`;
                                      `ConcurrentHashMap` overview; `PriorityQueue`;
                                      `Deque`; fail-fast vs weakly consistent iterators;
                                      `Comparable` vs `Comparator`; time/space complexity
                                      and ordering guarantees

  Learning resources                  [Dev.java --- Collections](https://dev.java/learn/);
                                      Java API documentation for
                                      [Map](https://docs.oracle.com/en/java/javase/);
                                      [HashMap](https://docs.oracle.com/en/java/javase/);
                                      Effective Java --- collections and equality

  Practical work                      Implement a simplified hash map with buckets and
                                      resizing; write comparator exercises; build an LRU
                                      cache using `LinkedHashMap`; benchmark list/map
                                      operations on different access patterns

  Completion criteria                 Draw the `HashMap` put/get/resize flow; explain
                                      collision handling and why mutable keys are
                                      dangerous; choose the correct collection for
                                      ordering, lookup, concurrency and memory needs

  Dependencies                        A1
  -----------------------------------------------------------------------------------------

### A3. Java 8+ and modern language features

  -------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Lambdas, functional interfaces,
                                      Streams, Optional, modern Java
                                      features

  Subtopics                           Functional interfaces; method
                                      references; stream pipeline/laziness;
                                      intermediate vs terminal operations;
                                      `map` vs `flatMap`;
                                      collectors/grouping/partitioning;
                                      parallel streams and common-pool
                                      risks; `Optional`
                                      usage/anti-patterns; records, sealed
                                      classes and pattern matching (know
                                      the version used by target
                                      employers); date/time API

  Learning resources                  [Dev.java --- Generics, Lambda and
                                      Pattern
                                      Matching](https://dev.java/learn/);
                                      [Dev.java --- Collections and
                                      Streams](https://dev.java/learn/)

  Practical work                      Transform a list of trades into
                                      grouped summaries; implement
                                      grouping, partitioning and
                                      aggregation using streams and loops;
                                      compare readability and performance

  Completion criteria                 Explain lazy evaluation, stream
                                      execution, `map`/`flatMap`, collector
                                      behavior and when not to use parallel
                                      streams

  Dependencies                        A1, A2
  -------------------------------------------------------------------------

### A4. Concurrency fundamentals and Java Memory Model

  -------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------
  Priority                            High

  Time estimate                       6 hours

  Topics                              Threads, synchronization, visibility,
                                      atomicity, happens-before

  Subtopics                           Thread lifecycle; race conditions;
                                      critical sections; intrinsic
                                      locks/`synchronized`; monitor lock;
                                      `volatile`; atomic classes and CAS;
                                      happens-before rules; safe
                                      publication;
                                      immutable/thread-confined state;
                                      deadlock, livelock and starvation;
                                      interrupt handling;
                                      `wait`/`notify`/`notifyAll`; spurious
                                      wakeups and condition predicates

  Learning resources                  [Java concurrency tutorials on
                                      Dev.java](https://dev.java/learn/);
                                      Java Language Specification ---
                                      Threads and Locks; Java Concurrency
                                      in Practice (selected chapters)

  Practical work                      Create a counter with a race; fix it
                                      with synchronization and atomics;
                                      implement producer-consumer with
                                      `wait`/`notify`; reproduce a
                                      visibility issue and explain why
                                      `volatile` changes the outcome

  Completion criteria                 Explain atomicity vs visibility vs
                                      ordering; identify a data race;
                                      explain happens-before; use `wait` in
                                      a condition loop; distinguish
                                      `volatile` from `synchronized` and
                                      atomic variables

  Dependencies                        A1
  -------------------------------------------------------------------------

### A5. Locks, executors and asynchronous programming

  ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Explicit locks, thread pools,
                                      futures and coordination

  Subtopics                           `ReentrantLock`; `ReadWriteLock`;
                                      `Condition` with `await`/`signal`;
                                      lock fairness; `tryLock`;
                                      `CountDownLatch`; `CyclicBarrier`;
                                      `Semaphore`; `BlockingQueue`;
                                      `ExecutorService`; thread-pool
                                      sizing and queue/rejection policies;
                                      `Future`; `CompletableFuture`;
                                      exception handling and composition;
                                      cancellation and timeouts; virtual
                                      threads and when they help

  Learning resources                  [Dev.java --- Virtual
                                      Threads](https://dev.java/learn/);
                                      Java API docs for
                                      `java.util.concurrent`; Java
                                      Concurrency in Practice

  Practical work                      Implement bounded producer-consumer
                                      using `BlockingQueue`; implement a
                                      rate-limited worker pool; compare
                                      fixed thread pools with virtual
                                      threads for blocking I/O; build a
                                      `CompletableFuture` fan-out/fan-in
                                      example

  Completion criteria                 Select a coordination primitive for
                                      a use case; explain pool saturation
                                      and rejection; avoid
                                      blocking/deadlock mistakes; explain
                                      virtual-thread benefits and
                                      limitations

  Dependencies                        A4
  ------------------------------------------------------------------------

### A6. JVM architecture and memory management

  --------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- --------------------------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              JVM runtime areas, class loading, garbage collection and
                                      memory troubleshooting

  Subtopics                           Heap generations/regions conceptually; stack frames;
                                      metaspace; program counter and native stack; object
                                      allocation; class loading/linking/initialization; parent
                                      delegation; JIT and tiered compilation; GC roots;
                                      reachability; common collectors (G1, ZGC at conceptual
                                      level); stop-the-world pauses; memory leaks vs high
                                      allocation; `OutOfMemoryError` types;
                                      `StackOverflowError`; escape analysis basics

  Learning resources                  [Java Virtual Machine
                                      Specification](https://docs.oracle.com/javase/specs/);
                                      [Dev.java --- JVM tools and
                                      troubleshooting](https://dev.java/learn/); Java Garbage
                                      Collection documentation

  Practical work                      Inspect a heap dump; generate controlled allocation
                                      pressure; use JFR/GC logs; compare stack vs heap
                                      examples; investigate a retained-object leak

  Completion criteria                 Draw JVM runtime memory areas; explain class loading and
                                      GC reachability; distinguish memory leak from allocation
                                      pressure; describe a systematic OOM investigation

  Dependencies                        A1
  --------------------------------------------------------------------------------------------

### A7. JVM diagnostics and performance

  --------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- --------------------------------------------
  Priority                            Medium

  Time estimate                       3 hours

  Topics                              Profiling, diagnostics and performance
                                      reasoning

  Subtopics                           JFR; `jcmd`, `jstack`, `jmap`; GC logs;
                                      heap/thread dumps; CPU vs allocation vs lock
                                      contention; latency percentiles; warm-up/JIT
                                      effects; microbenchmark pitfalls and JMH
                                      awareness

  Learning resources                  [Dev.java --- JFR, monitoring and
                                      troubleshooting](https://dev.java/learn/);
                                      OpenJDK JMH documentation

  Practical work                      Profile a deliberately slow program;
                                      identify a hot method, allocation hotspot
                                      and blocked thread; write a short
                                      incident-style diagnosis

  Completion criteria                 Select the right diagnostic artifact for
                                      CPU, memory, deadlock and latency symptoms;
                                      avoid claiming a microbenchmark is
                                      representative without controls

  Dependencies                        A6, A5
  --------------------------------------------------------------------------------

### A8. Spring and Spring Boot internals

  -----------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              IoC/DI, bean lifecycle, proxies, AOP, configuration and
                                      transactions

  Subtopics                           Bean definitions and scopes; constructor injection; bean
                                      lifecycle callbacks; component scanning/autoconfiguration;
                                      configuration properties; proxy-based AOP; JDK vs CGLIB proxies;
                                      self-invocation limitation; `@Transactional`
                                      propagation/isolation/rollback rules; transaction boundaries;
                                      lazy loading; application events; filters vs interceptors vs AOP

  Learning resources                  [Spring Framework
                                      Reference](https://docs.spring.io/spring-framework/reference/);
                                      Spring Boot Reference Documentation

  Practical work                      Build a small Spring Boot service; inspect bean initialization;
                                      demonstrate transactional rollback and propagation; reproduce
                                      self-invocation proxy behavior

  Completion criteria                 Explain how dependency injection and proxying work; diagnose why
                                      `@Transactional` may not apply; choose transaction boundaries and
                                      propagation intentionally

  Dependencies                        A1, A4
  -----------------------------------------------------------------------------------------------------

### A9. Persistence, SQL and transaction fundamentals

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Relational database behavior from
                                      an application engineer's
                                      perspective

  Subtopics                           ACID; isolation levels;
                                      dirty/non-repeatable/phantom reads;
                                      optimistic vs pessimistic locking;
                                      indexes and B-tree intuition;
                                      composite index ordering; query
                                      plans; N+1 queries; connection
                                      pools; transaction boundaries;
                                      idempotency keys; outbox pattern
                                      overview

  Learning resources                  PostgreSQL official documentation;
                                      Use The Index, Luke; Spring
                                      transaction reference

  Practical work                      Create tables and indexes; compare
                                      query plans with `EXPLAIN ANALYZE`;
                                      reproduce lost updates; implement
                                      optimistic locking and an
                                      idempotent API

  Completion criteria                 Explain isolation anomalies; read a
                                      basic query plan; choose
                                      optimistic/pessimistic locking;
                                      explain why external calls inside
                                      DB transactions are dangerous

  Dependencies                        A8; basic SQL knowledge
  -----------------------------------------------------------------------

### A10. Testing, API engineering and production concerns

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            Medium

  Time estimate                       3 hours

  Topics                              Reliable service implementation

  Subtopics                           Unit vs integration vs contract
                                      tests; JUnit 5; Mockito;
                                      Testcontainers awareness; REST
                                      semantics; validation; exception
                                      mapping; API versioning; timeouts;
                                      retries; circuit breakers;
                                      structured logging; metrics and
                                      tracing; secrets and configuration

  Learning resources                  JUnit 5 User Guide; Mockito
                                      documentation; Spring Boot testing
                                      reference; OpenTelemetry
                                      documentation

  Practical work                      Test a Spring service with a real
                                      database container; test
                                      retry/idempotency behavior; add
                                      correlation IDs and metrics

  Completion criteria                 Design a test pyramid for a
                                      service; distinguish mock-based
                                      confidence from integration
                                      confidence; explain
                                      timeout/retry/circuit-breaker
                                      interactions

  Dependencies                        A8, A9
  -----------------------------------------------------------------------

### Java track completion standard

By the end of six months, be able to:

-   Explain `HashMap`, `ConcurrentHashMap`, equality/hash contracts and
    collection trade-offs without memorized scripts.
-   Reason about Java Memory Model, happens-before, locks, conditions,
    executors and thread safety.
-   Explain JVM memory areas, class loading, garbage collection and a
    practical troubleshooting approach.
-   Diagnose common Spring proxy/transaction issues.
-   Discuss database isolation, indexes, idempotency and production
    service reliability.
-   Write small code examples during an interview and defend the
    trade-offs.

------------------------------------------------------------------------

# 4. Track B --- Data Structures & Algorithms (NeetCode 250 only)

## Track objective

Complete and internalize the official NeetCode 250 problem set. The goal
is not to accumulate solved counts; it is to recognize patterns, derive
solutions, implement cleanly in Java, and explain complexity.

**Official sheet:** [NeetCode
250](https://neetcode.io/practice/practice/neetcode250)\
**Official composition:** 250 problems --- 60 Easy, 155 Medium, 35
Hard.\
**Official topic groups and counts:** listed below.\
**Estimated remaining effort:** depends on the imported completion
state. For a learner with roughly 89 already completed, plan around 6--8
new problems per week on average, plus revision; adjust based on actual
unsolved count and difficulty.

### Strict DSA rules for the application

-   The NeetCode 250 page is the only source of new DSA problems in this
    track.
-   Store the exact official problem title, URL, topic group, difficulty
    and current completion state.
-   Preserve official topic ordering. The study planner may schedule a
    revision from a previously completed problem, but must not add an
    external problem.
-   If a problem is already solved, schedule it for revision only if
    recall/implementation confidence is weak.
-   Do not mark a problem complete after reading a solution. Completion
    requires an independent attempt and a post-solve explanation.
-   If stuck, use progressive hints. Reveal the full solution only after
    a genuine attempt or an explicit request.

### B1. Official topic sequence

The following is the official NeetCode 250 category order and problem
count. Subtopics below describe the patterns to learn within that
official category; they are not extra problem lists.

  ----------------------------------------------------------------------------------------------------------
        Order Official topic    Problems Priority          Time Subtopics / patterns        Dependencies
                                                       estimate                             
  ----------- -------------- ----------- ---------- ----------- --------------------------- ----------------
            1 Arrays &                22 High            5--7 h Frequency maps; sets;       Java arrays,
              Hashing                                           duplicate detection;        strings, HashMap
                                                                grouping; prefix/suffix     
                                                                products; anagram           
                                                                signatures; array           
                                                                invariants                  

            2 Two Pointers            13 High            3--4 h Opposite-direction          Arrays & Hashing
                                                                pointers; same-direction    
                                                                pointers; sorted-array      
                                                                reasoning; in-place         
                                                                transformations;            
                                                                partitioning                

            3 Sliding Window           9 High            3--4 h Fixed vs variable windows;  Arrays, HashMap,
                                                                frequency-count windows;    Two Pointers
                                                                shrink/expand invariant;    
                                                                longest/shortest valid      
                                                                window                      

            4 Binary Search           14 High            4--5 h Search space; lower/upper   Sorted arrays;
                                                                bound; rotated arrays;      loop invariants
                                                                boundary conditions; binary 
                                                                search on answer            

            5 Linked List             14 High            4--5 h Reversal; fast/slow         References, Two
                                                                pointers; cycle detection;  Pointers
                                                                merge; dummy nodes; pointer 
                                                                manipulation                

            6 Stack                   14 High            4--5 h Monotonic stack;            Arrays, basic
                                                                parentheses; expression     data structures
                                                                evaluation; next            
                                                                greater/smaller; stack      
                                                                simulation                  

            7 Trees                   23 High            7--9 h DFS/BFS; recursion; level   Recursion,
                                                                order; BST invariants; path Queue, Stack
                                                                problems; construction;     
                                                                serialization; tree DP      
                                                                intuition                   

            8 Tries                    4 Medium        1.5--2 h Prefix tree;                Trees,
                                                                insert/search/startsWith;   HashMap/arrays
                                                                wildcard traversal; trie    
                                                                for word search             

            9 Backtracking            17 High            5--7 h Decision tree;              Recursion, Trees
                                                                choose/explore/unchoose;    
                                                                subsets; permutations;      
                                                                combinations; pruning;      
                                                                duplicate handling          

           10 Heap /                  12 High            4--5 h Top-K; min/max heap; two    Arrays, HashMap,
              Priority Queue                                    heaps; scheduling; k-way    comparator
                                                                merge; custom comparator    

           11 Graphs                  21 High            7--9 h Adjacency list/matrix;      Trees, Queue,
                                                                BFS/DFS; connected          Stack
                                                                components; grid traversal; 
                                                                cycle detection;            
                                                                topological sort; indegree  

           12 Advanced                10 Medium          4--5 h Union-Find/DSU; path        Graphs, Heap
              Graphs                                            compression; rank/size;     
                                                                Dijkstra; minimum spanning  
                                                                tree; shortest-path         
                                                                trade-offs                  

           13 1-D Dynamic             17 High            6--8 h State definition;           Recursion,
              Programming                                       recurrence; base cases;     Arrays
                                                                memoization/tabulation;     
                                                                space optimization;         
                                                                subsequence/state           
                                                                transitions                 

           14 2-D Dynamic             16 Medium          6--8 h Grid DP; two-sequence DP;   1-D DP
              Programming                                       knapsack-style state;       
                                                                interval/state dimensions;  
                                                                transition order            

           15 Greedy                  14 Medium          4--5 h Local choice; exchange      Sorting,
                                                                argument; sorting;          intervals, heap
                                                                scheduling; proof           
                                                                intuition; heap-assisted    
                                                                greedy                      

           16 Intervals                7 Medium          2--3 h Sorting endpoints; overlap; Sorting, greedy
                                                                merge; sweep line;          
                                                                meeting-room patterns       

           17 Bit                     10 Medium          3--4 h AND/OR/XOR; shifts; masks;  Integer
              Manipulation                                      set/clear/test bit; two's   representation
                                                                complement; bit-count       
                                                                reasoning                   

           18 Math &                  13 Medium          4--5 h GCD; primes; modular        Basic math,
              Geometry                                          arithmetic basics;          arrays
                                                                coordinates; slopes;        
                                                                rotation; matrix geometry   
  ----------------------------------------------------------------------------------------------------------

**Important:** the official sheet has 19? Verify the live sheet before
import. The current official page surfaced 18 categories in its visible
category list; the app should treat the live official page as canonical
if NeetCode changes the taxonomy.

### B2. Per-problem workflow

  -----------------------------------------------------------------------
  Step                    Action                  Completion evidence
  ----------------------- ----------------------- -----------------------
  1\. Understand          Restate input, output,  Can explain the problem
                          constraints and edge    without re-reading
                          cases                   

  2\. Brute force         Identify the simplest   State its complexity
                          correct approach        

  3\. Pattern recognition Identify the relevant   Explain why the pattern
                          data                    fits
                          structure/invariant     

  4\. Implement           Code in Java without    Code compiles and
                          copying the solution    passes tests

  5\. Analyze             Time and space          Explain worst-case
                          complexity              behavior

  6\. Review              Compare with the        Record one insight or
                          official                missed edge case
                          explanation/solution    

  7\. Reproduce           Re-solve later without  Independent solution
                          notes                   within target time
  -----------------------------------------------------------------------

### B3. Problem completion criteria

A problem is **Solved** only when: - The learner has made a genuine
independent attempt. - The final Java implementation passes the official
test cases. - Time and space complexity are understood. - The learner
can explain the key invariant or recurrence. - At least one edge case is
identified.

A problem is **Interview ready** when: - It can be solved again without
viewing the solution. - The learner can explain brute force and
optimized approaches. - The learner can defend complexity and
implementation choices. - The learner can handle a reasonable follow-up
or variation.

### B4. Revision policy

Use a retrieval-based schedule rather than repeatedly rereading
solutions.

  -----------------------------------------------------------------------
  Review                  When                    What to do
  ----------------------- ----------------------- -----------------------
  R1                      Next day                Explain pattern and
                                                  complexity from memory;
                                                  inspect code only after
                                                  recall

  R2                      Day 3--4                Re-derive the approach;
                                                  write key invariant

  R3                      Day 7                   Re-solve or write
                                                  pseudocode under a time
                                                  limit

  R4                      Day 14                  Solve without notes;
                                                  identify the first
                                                  point of hesitation

  R5                      Day 30                  Timed re-solve or
                                                  explain to an
                                                  interviewer

  R6                      Day 60--90              Final retention check
                                                  for high-priority/weak
                                                  problems
  -----------------------------------------------------------------------

If a revision fails, reset the interval to 1 day and mark the concept as
weak. Do not reset the entire topic.

### B5. Six-month DSA pacing

This is a pacing guide, not permission to skip official order. Start at
the first incomplete official problem in the current sheet order, and
let the sheet's prerequisites govern sequencing.

  ------------------------------------------------------------------------
  Phase                                        Weeks Target
  --------------------- ---------------------------- ---------------------
  Foundation and                                1--4 Work through the
  pattern refresh                                    earliest incomplete
                                                     categories; focus on
                                                     arrays/hash, two
                                                     pointers, sliding
                                                     window, binary search
                                                     and linked lists as
                                                     applicable

  Core structures                               5--9 Continue official
                                                     order through stack,
                                                     trees, tries,
                                                     backtracking and
                                                     heaps

  Graph depth                                 10--14 Complete graph and
                                                     advanced graph
                                                     categories; revise
                                                     earlier weak patterns

  DP and optimization                         15--19 Complete 1-D and 2-D
                                                     DP; then greedy and
                                                     intervals

  Final categories                            20--22 Bit manipulation and
                                                     math/geometry; close
                                                     remaining
                                                     official-sheet
                                                     problems

  Consolidation                               23--26 Re-solve weak/flagged
                                                     NeetCode 250
                                                     problems, timed mixed
                                                     practice using only
                                                     the sheet, and mock
                                                     interviews
  ------------------------------------------------------------------------

**Weekly target:** approximately 6--8 new problems if about 160 remain,
with 3--5 revisions. If more remain, increase only if the average
problem is genuinely understood; otherwise extend completion beyond the
six-month window rather than rushing.

------------------------------------------------------------------------

# 5. Track C --- Low-Level Design (CodeArmy)

## Track objective

Become capable of taking an open-ended machine-coding/LLD prompt,
clarifying requirements, modeling domain entities, defining interfaces,
applying patterns only where justified, and implementing clean, testable
Java code under time constraints.

**Primary curriculum:** [Coder Army LLD
playlist](https://youtube.com/playlist?list=PLQEaRBV9gAFvzp6XhcNFpk1WdOcyVo9qT)\
**Companion implementation repository:** [Coder Army LLD course
code](https://github.com/adityatandon15/Low-Level-Design-Course)

The repository organizes lecture code. Follow the playlist's actual
lecture sequence and map each lecture to the learning units below. The
roadmap adds deliberate practice because watching a walkthrough is not
equivalent to independently designing a solution.

**Estimated total:** 30--36 focused hours across six months.

### C1. LLD approach and object-oriented foundations

  --------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- --------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Problem clarification, OOP and UML

  Subtopics                           Functional/non-functional
                                      requirements; use cases; actors;
                                      entities/value objects; encapsulation;
                                      abstraction; inheritance;
                                      polymorphism; composition;
                                      association/aggregation/composition;
                                      class diagrams; sequence diagrams;
                                      object collaboration

  Learning resources                  CodeArmy playlist --- introduction and
                                      OOP fundamentals; companion GitHub
                                      lecture notes/code

  Practical work                      Model a library, order and
                                      notification domain; draw class and
                                      sequence diagrams before coding

  Completion criteria                 Convert a prompt into requirements,
                                      entities, responsibilities and a class
                                      diagram; explain why composition may
                                      be preferable to inheritance

  Dependencies                        Java A1
  --------------------------------------------------------------------------

### C2. Design principles and code quality

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              SOLID and related principles

  Subtopics                           SRP; OCP; LSP; ISP; DIP; DRY; KISS;
                                      YAGNI; cohesion; coupling;
                                      dependency inversion; programming
                                      to interfaces; separation of
                                      concerns

  Learning resources                  CodeArmy playlist --- design
                                      principles/SOLID; Effective Java;
                                      Refactoring.Guru --- design
                                      principles

  Practical work                      Refactor a large service class into
                                      cohesive collaborators; create a
                                      deliberately rigid design and
                                      improve it

  Completion criteria                 Identify a violation, explain its
                                      consequence, and refactor without
                                      overengineering

  Dependencies                        C1
  -----------------------------------------------------------------------

### C3. Creational patterns

  -------------------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Object creation patterns

  Subtopics                           Simple factory; Factory Method; Abstract Factory; Builder; Singleton;
                                      Prototype; lazy/eager creation; thread-safety and lifecycle concerns

  Learning resources                  CodeArmy playlist --- creational patterns; [Refactoring.Guru ---
                                      Creational
                                      Patterns](https://refactoring.guru/design-patterns/creational-patterns)

  Practical work                      Implement a notification-provider factory, builder for a complex request,
                                      and a thread-safe singleton; explain when a DI container makes manual
                                      singleton unnecessary

  Completion criteria                 Recognize the creation problem each pattern solves; implement one without
                                      following a tutorial; discuss trade-offs

  Dependencies                        C1, C2, Java A2/A4
  -------------------------------------------------------------------------------------------------------------

### C4. Structural patterns

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            Medium

  Time estimate                       3 hours

  Topics                              Composition and interface
                                      adaptation

  Subtopics                           Adapter; Decorator; Facade; Proxy;
                                      Composite; Bridge; Flyweight;
                                      object composition; wrapper design;
                                      remote/protection/virtual proxy

  Learning resources                  CodeArmy playlist --- structural
                                      patterns; Refactoring.Guru ---
                                      Structural Patterns

  Practical work                      Add caching and authorization
                                      decorators to a service; adapt two
                                      payment-provider APIs to a common
                                      interface; model a file system
                                      using Composite

  Completion criteria                 Explain the structural problem
                                      being solved and avoid using a
                                      pattern where a simple class is
                                      clearer

  Dependencies                        C1, C2
  -----------------------------------------------------------------------

### C5. Behavioral patterns

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Object collaboration and behavior
                                      variation

  Subtopics                           Strategy; Observer; Command; State;
                                      Chain of Responsibility; Template
                                      Method; Iterator; Mediator;
                                      Visitor; Memento; Null Object;
                                      event notification; state
                                      transitions

  Learning resources                  CodeArmy playlist --- behavioral
                                      patterns; Refactoring.Guru ---
                                      Behavioral Patterns

  Practical work                      Implement a payment strategy,
                                      notification observer,
                                      vending-machine state machine,
                                      command-based undo, and validation
                                      chain

  Completion criteria                 Choose a pattern based on changing
                                      behavior or collaboration; explain
                                      lifecycle and extension points;
                                      write unit tests

  Dependencies                        C1, C2, C3
  -----------------------------------------------------------------------

### C6. End-to-end LLD projects from the playlist

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       8--10 hours

  Topics                              Complete machine-coding problems

  Subtopics                           Work through the playlist's
                                      real-world design projects in its
                                      published order. Projects
                                      represented in the course include
                                      document editor/Google Docs
                                      concepts, notification engine,
                                      music player, file system, payment
                                      gateway, discount/coupon engine,
                                      quick-commerce/inventory, dating
                                      application, Splitwise, vending
                                      machine, Tic-Tac-Toe, Snake and
                                      Ladder, chat room and chess. Verify
                                      the exact project list and lecture
                                      titles against the current
                                      playlist.

  Learning resources                  CodeArmy playlist and companion
                                      repository

  Practical work                      For each selected project: clarify
                                      requirements; write use cases; draw
                                      class diagram; define interfaces;
                                      implement core flows; add tests;
                                      add one extension requirement

  Completion criteria                 At least 4 substantial projects
                                      independently implemented; can
                                      explain the design, demonstrate
                                      core flows and add a new
                                      requirement without rewriting
                                      everything

  Dependencies                        C1--C5
  -----------------------------------------------------------------------

### C7. Machine-coding readiness: testing, concurrency and extensibility

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Production-minded LLD

  Subtopics                           Unit testing; dependency injection;
                                      thread-safety; concurrent access;
                                      synchronization boundaries;
                                      immutable state; validation; error
                                      handling; persistence abstraction;
                                      extensibility; clean package
                                      structure; time-boxing

  Learning resources                  JUnit 5; Java concurrency track
                                      A4--A5; selected CodeArmy
                                      implementations

  Practical work                      Add concurrent booking or inventory
                                      updates; write tests for invalid
                                      transitions and race conditions;
                                      implement a new provider without
                                      changing business logic

  Completion criteria                 Complete a 60--90 minute
                                      machine-coding exercise with
                                      requirements, working code, tests
                                      and clear explanation

  Dependencies                        C1--C6, Java A4/A5
  -----------------------------------------------------------------------

### LLD practice protocol

For every project:

1.  Spend 10 minutes clarifying requirements and identifying scope.
2.  Spend 10--15 minutes modeling entities and relationships.
3.  Define public APIs and core interfaces before implementation.
4.  Implement the smallest correct vertical slice.
5.  Add tests for happy path, invalid input and one edge case.
6.  Review whether patterns improved the design or merely added
    abstraction.
7.  Revisit the design after adding one new requirement.

**Do not copy the repository implementation line by line.** Use it after
your own attempt to compare design choices.

------------------------------------------------------------------------

# 6. Track D --- High-Level Design (HLD)

## Track objective

Design reliable, scalable backend systems and communicate the reasoning
clearly. At a senior SDE level, the interviewer is evaluating
requirements discovery, decomposition, data modeling, bottleneck
identification, failure handling, and trade-off judgment---not the
number of buzzwords used.

**Estimated total:** 35--42 focused hours across six months.

### D1. System design interview method and requirements

  ---------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ---------------------------------------------------------------
  Priority                            High

  Time estimate                       2 hours

  Topics                              Problem framing and estimation

  Subtopics                           Functional vs non-functional requirements; scope; user
                                      journeys; APIs; constraints; traffic estimates; DAU/MAU; QPS;
                                      peak vs average; read/write ratio; storage growth; bandwidth;
                                      latency SLOs

  Learning resources                  [System Design
                                      Primer](https://github.com/donnemartin/system-design-primer);
                                      NeetCode system design course material if available

  Practical work                      For URL shortener and notification system, write requirements
                                      and estimate scale before drawing architecture

  Completion criteria                 Start a design interview with clarifying questions and defend
                                      rough capacity estimates

  Dependencies                        None
  ---------------------------------------------------------------------------------------------------

### D2. Networking and request lifecycle

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              How requests travel through a
                                      distributed system

  Subtopics                           DNS; TCP handshake; TLS; HTTP/1.1
                                      vs HTTP/2 vs HTTP/3 at a conceptual
                                      level; REST; gRPC; connection
                                      pooling; keep-alive; reverse proxy;
                                      load balancer; CDN; API gateway;
                                      timeouts; retries

  Learning resources                  System Design Primer ---
                                      communication; MDN HTTP guides;
                                      Cloudflare learning center

  Practical work                      Trace a request from browser/client
                                      to load balancer, service and
                                      database; identify where TLS,
                                      authentication, retries and
                                      timeouts apply

  Completion criteria                 Explain the request path and
                                      distinguish DNS, CDN, reverse
                                      proxy, API gateway and load
                                      balancer

  Dependencies                        D1
  -----------------------------------------------------------------------

### D3. Data modeling and database selection

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Relational and non-relational data
                                      design

  Subtopics                           SQL vs NoSQL;
                                      normalization/denormalization;
                                      indexes; composite indexes; query
                                      patterns; transactions; ACID;
                                      isolation; replication; read
                                      replicas; partitioning; sharding;
                                      consistent hashing; hot partitions;
                                      schema evolution

  Learning resources                  System Design Primer --- database;
                                      PostgreSQL documentation; Use The
                                      Index, Luke

  Practical work                      Model data for chat, feed and order
                                      processing; choose keys and indexes
                                      based on access patterns; explain
                                      read/write scaling

  Completion criteria                 Select storage based on access
                                      patterns and consistency
                                      requirements; explain indexing,
                                      replication and sharding trade-offs

  Dependencies                        D1, D2
  -----------------------------------------------------------------------

### D4. Caching and content delivery

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Caching strategy

  Subtopics                           Cache-aside; read-through;
                                      write-through; write-behind;
                                      refresh-ahead; TTL; invalidation;
                                      eviction (LRU/LFU); cache stampede;
                                      hot keys; negative caching; CDN;
                                      browser cache; consistency and
                                      stale reads

  Learning resources                  System Design Primer --- cache;
                                      Redis documentation

  Practical work                      Add Redis cache to a sample read
                                      API; compare hit/miss latency;
                                      demonstrate invalidation and
                                      stampede mitigation

  Completion criteria                 Choose a caching strategy and
                                      explain stale-data, invalidation
                                      and failure trade-offs

  Dependencies                        D3
  -----------------------------------------------------------------------

### D5. Asynchronous processing and messaging

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Event-driven architecture

  Subtopics                           Queue vs pub/sub vs event log;
                                      Kafka concepts (topic, partition,
                                      offset, consumer group); ordering;
                                      at-least-once delivery;
                                      at-most-once; effectively-once
                                      processing; idempotency; retries;
                                      exponential backoff; DLQ; poison
                                      messages; backpressure; consumer
                                      lag; outbox pattern; saga basics

  Learning resources                  Kafka documentation; System Design
                                      Primer --- asynchronism;
                                      microservices.io patterns

  Practical work                      Design an order/payment event flow;
                                      implement idempotent consumer
                                      behavior; diagram retry and DLQ
                                      handling

  Completion criteria                 Explain delivery semantics and why
                                      idempotency matters; design a
                                      recoverable asynchronous workflow

  Dependencies                        D3
  -----------------------------------------------------------------------

### D6. Distributed systems fundamentals

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Consistency, coordination and
                                      failure

  Subtopics                           CAP theorem and its practical
                                      limits; consistency models;
                                      eventual consistency; quorum
                                      reads/writes; leader/follower;
                                      replication lag; split brain;
                                      leases; clocks and ordering;
                                      distributed locks; failure
                                      detectors; retries and duplicate
                                      requests; partial failure;
                                      idempotency

  Learning resources                  System Design Primer; Designing
                                      Data-Intensive Applications by
                                      Martin Kleppmann (selected
                                      chapters); Google SRE book

  Practical work                      Explain a stale read scenario;
                                      design a safe idempotent transfer
                                      workflow; identify failure modes in
                                      a distributed lock design

  Completion criteria                 Reason about consistency and
                                      availability under failures;
                                      explain why distributed systems
                                      cannot be treated as one reliable
                                      process

  Dependencies                        D3, D5
  -----------------------------------------------------------------------

### D7. Scalability, availability and resilience

  ------------------------------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Scaling and fault tolerance

  Subtopics                           Vertical/horizontal scaling; stateless services; load balancing; autoscaling;
                                      replication; sharding; failover; health checks; circuit breakers; bulkheads; rate
                                      limiting; quotas; graceful degradation; multi-region concepts; disaster recovery;
                                      RTO/RPO

  Learning resources                  [AWS Well-Architected
                                      Framework](https://docs.aws.amazon.com/wellarchitected/latest/userguide/waf.html);
                                      Google SRE book; System Design Primer

  Practical work                      Add rate limiting and circuit breaker to a sample service; design a failure-mode
                                      table for a payment API

  Completion criteria                 Identify bottlenecks and failure domains; explain availability/recovery trade-offs
                                      and graceful degradation

  Dependencies                        D2--D6
  ------------------------------------------------------------------------------------------------------------------------

### D8. Observability, security and operations

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Operating a distributed system

  Subtopics                           Logs, metrics and traces;
                                      correlation/trace IDs; RED and USE
                                      methods; SLI/SLO/SLA; alerting;
                                      dashboards; audit logs;
                                      authentication vs authorization;
                                      OAuth/JWT concepts; encryption in
                                      transit/at rest; secret management;
                                      data privacy; abuse prevention

  Learning resources                  Google SRE book; OpenTelemetry
                                      documentation; AWS Well-Architected
                                      security and operational excellence
                                      pillars

  Practical work                      Define SLIs/SLOs for a service;
                                      draw a trace across API, queue and
                                      worker; add a threat-model
                                      checklist

  Completion criteria                 Define actionable metrics and
                                      alerts; identify security
                                      boundaries and operational risks

  Dependencies                        D2, D7
  -----------------------------------------------------------------------

### D9. System design case studies

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       8--10 hours

  Topics                              End-to-end architecture exercises

  Subtopics                           URL shortener; notification
                                      service; rate limiter; news/feed
                                      timeline; chat/messaging; file
                                      storage; ticket booking;
                                      payment/order processing;
                                      search/autocomplete; job scheduler.
                                      Prioritize 5--6 designs deeply
                                      rather than memorizing every
                                      solution.

  Learning resources                  System Design Primer --- real-world
                                      designs and interview questions;
                                      Alex Xu, System Design Interview
                                      (selected cases); engineering blogs
                                      from target companies

  Practical work                      For each case: requirements,
                                      estimates, APIs, data model,
                                      high-level diagram, bottlenecks,
                                      failure modes, observability,
                                      security and trade-offs

  Completion criteria                 Complete at least 5 designs in
                                      40--50 minutes each; handle
                                      interviewer changes and explain at
                                      least two alternatives

  Dependencies                        D1--D8
  -----------------------------------------------------------------------

### D10. Senior-level architecture judgment

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            Medium

  Time estimate                       3 hours

  Topics                              Evolution and organizational
                                      trade-offs

  Subtopics                           Monolith vs microservices; service
                                      boundaries; synchronous vs
                                      asynchronous calls; consistency vs
                                      availability; build vs buy;
                                      operational complexity; migration
                                      strategy; backward compatibility;
                                      schema/API evolution;
                                      cost/performance/reliability
                                      trade-offs

  Learning resources                  AWS Well-Architected; engineering
                                      blogs; Designing Data-Intensive
                                      Applications

  Practical work                      Write an architecture decision
                                      record for a monolith-to-service
                                      extraction; identify when not to
                                      use microservices

  Completion criteria                 Explain not only what architecture
                                      you choose but why alternatives are
                                      worse for the stated constraints

  Dependencies                        D1--D9
  -----------------------------------------------------------------------

### HLD interview response template

For every design question, use this sequence:

1.  Clarify scope and requirements.
2.  Estimate scale and identify constraints.
3.  Define APIs and core data model.
4.  Draw the simplest working architecture.
5.  Walk through read/write flows.
6.  Identify bottlenecks and scale the relevant component.
7.  Discuss consistency, failures, retries and idempotency.
8.  Add observability, security and operational concerns.
9.  State trade-offs and what you would change at 10× scale.

------------------------------------------------------------------------

# 7. Track E --- AI Engineering Upskilling for SDE II (Agentic AI)

## Track objective

Upskill from an experienced backend engineer to one who understands how
LLMs work, can build and evaluate LLM-powered features, and can design,
secure and operate agentic systems. The goal is to be a confident SDE II
in a world where AI is part of the standard backend toolbox: you should
know what a model can and cannot do, how to wire it into real systems,
how to prove it works, and how to use AI tools to ship faster without
losing engineering judgment.

**Not the goal:** training foundation models, deep mathematical ML
research, or collecting agent frameworks without building working
systems.

**Estimated total:** 55--60 focused hours across six months (about
2--2.25 hours per week), including one capstone project.\
**Suggested stack:** Python + FastAPI, one primary model provider API,
PostgreSQL/pgvector, MCP, Docker. Learn LangGraph only after building
the agent loop by hand. Do not learn every vendor and framework
simultaneously.

**Learning order principle:** concept, then raw API/code, then
framework. Every abstraction (LangChain, LangGraph, vector DB client,
MCP SDK) is learned after you have done the same thing manually once.

### Topic coverage map

  ------------------------------------------------------------------------
  Topic                                              Module
  -------------------------------------------------- ---------------------
  Python                                             E1

  LLMs, internals, tokens, context window            E2

  Limitations and hallucinations                     E2, E8, E11

  Streaming, API integration                         E3

  System prompts, context engineering                E4

  JSON / structured output                           E5

  Tool calling, MCP                                  E6

  Embeddings, vector databases, retrieval            E7

  RAG, chunking, citations                           E8

  Evals                                              E9

  Agents, workflows, memory, LangGraph               E10

  Security and guardrails                            E11

  Cost, latency, observability                       E12

  AI-assisted development                            E13

  Capstone and interview readiness                   E14
  ------------------------------------------------------------------------

### E1. Python for AI engineering

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Practical Python for an experienced Java engineer

  Subtopics                           Syntax and idioms vs Java (dynamic/duck typing, indentation, None,
                                      truthiness); list/dict/set/tuple; comprehensions; functions,
                                      *args/**kwargs; classes and dataclasses; modules and packages;
                                      exceptions and context managers; iterators and generators; decorators;
                                      type hints; virtual environments and uv/pip; Pydantic models;
                                      async/await and asyncio; HTTP clients (httpx); FastAPI basics; pytest;
                                      Jupyter notebooks for experiments

  Learning resources                  [Python tutorial](https://docs.python.org/3/tutorial/); [FastAPI
                                      tutorial](https://fastapi.tiangolo.com/tutorial/); [Pydantic
                                      docs](https://docs.pydantic.dev/); [uv docs](https://docs.astral.sh/uv/)

  Practical work                      Build a typed FastAPI endpoint with Pydantic validation, an async call
                                      to an external HTTP API, error handling and pytest tests

  Completion criteria                 Build and test a small API without relying on generated code for basic
                                      syntax or debugging; explain how Python async differs from Java
                                      threads/CompletableFuture

  Dependencies                        None; Java/Spring experience transfers for OOP, DI and REST concepts
  ----------------------------------- ------------------------------------------------------------------------

### E2. LLM foundations, internals, limitations and hallucinations

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              How LLMs work well enough to predict their behavior and failure modes

  Subtopics                           What an LLM is (next-token prediction); training stages (pretraining,
                                      supervised fine-tuning, preference tuning/RLHF) at concept level; tokens
                                      and tokenization (BPE, why tokens are not words, cost of code and
                                      non-English text); embeddings inside the model; transformer and
                                      self-attention intuition; context window (input + output share it,
                                      quadratic attention cost, long-context degradation, lost-in-the-middle);
                                      KV cache and why input vs output tokens differ in latency and price;
                                      sampling (temperature, top-p, top-k) and non-determinism; model
                                      families, sizes and reasoning models; knowledge cutoff; hosted vs
                                      open-weight models; limitations (hallucination, stale knowledge, weak
                                      counting/arithmetic, prompt sensitivity, sycophancy, long-context
                                      degradation); why hallucinations happen and their types (fabricated
                                      facts, wrong citations, ungrounded summaries, tool-result misreading)

  Learning resources                  Andrej Karpathy: Intro to Large Language Models and Let's build the GPT
                                      Tokenizer (YouTube); 3Blue1Brown neural network and transformer videos;
                                      [Hugging Face LLM Course](https://huggingface.co/learn/llm-course)
                                      (tokenizers chapter); Chip Huyen, AI Engineering (foundation models
                                      chapter); provider model documentation

  Practical work                      Notebook: count tokens for English, code, JSON and non-English text and
                                      compare; run one prompt at several temperatures; write 15 prompts that
                                      provoke hallucination and record how each model fails. Optional depth:
                                      implement scaled dot-product attention in NumPy

  Completion criteria                 Explain without notes how a prompt becomes tokens and then output, how
                                      the context window is consumed, why tokens drive cost and latency, and
                                      why hallucinations occur; estimate the token cost of a request

  Dependencies                        E1 for notebooks; no ML math background required
  ----------------------------------- ------------------------------------------------------------------------

### E3. LLM API integration, streaming and context-window management

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Calling models reliably from a backend service

  Subtopics                           Request/response anatomy; roles and message arrays; APIs are stateless,
                                      so conversation state lives in your app; streaming (server-sent events,
                                      delta events, cancellation, partial output); max tokens and stop
                                      reasons; timeouts; retries with exponential backoff and jitter; 429 and
                                      5xx handling; rate limits; token usage and cost estimation; context
                                      budgeting (truncation, sliding window, summarization); prompt caching
                                      concepts; provider abstraction; secrets management

  Learning resources                  [OpenAI API docs](https://developers.openai.com/api/docs/); [Anthropic
                                      docs](https://docs.claude.com); documentation of the one provider you
                                      choose as primary

  Practical work                      Build a small model gateway in FastAPI: streaming endpoint (SSE) with a
                                      simple client, usage and cost logging, timeout/retry behavior,
                                      configurable model, and a chat that trims history to a token budget

  Completion criteria                 Explain the request lifecycle, streaming mechanics and cost drivers;
                                      integrate an LLM API safely with bounded retries; show the app behaving
                                      correctly when the context budget is exceeded

  Dependencies                        E1, E2
  ----------------------------------- ------------------------------------------------------------------------

### E4. Prompting, system prompts and context engineering

  ----------------------------------- -------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------------------------------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Controlling model behavior through input design

  Subtopics                           System vs developer vs user messages and instruction hierarchy; structure of a good system
                                      prompt (role, task, constraints, output format, examples); few-shot examples; delimiters
                                      and XML-style sections; step-by-step reasoning and when reasoning models make it
                                      unnecessary; task decomposition; context engineering (what to include, ordering,
                                      compression, just-in-time retrieval); grounding and permission to say I don't know to
                                      reduce hallucination; prompt injection awareness; storing and versioning prompts like code;
                                      iterating against test cases rather than intuition

  Learning resources                  [Anthropic prompt engineering
                                      overview](https://docs.claude.com/en/docs/build-with-claude/prompt-engineering/overview);
                                      [Anthropic: Effective context engineering for AI
                                      agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents);
                                      OpenAI prompt engineering guide

  Practical work                      Write a system prompt for a domain assistant; compare three prompt variants on 15 fixed
                                      test inputs; keep prompts in version control with a changelog

  Completion criteria                 Explain why each prompt element exists; show measured before/after improvement from a
                                      prompt change; explain why prompting alone does not guarantee correctness

  Dependencies                        E3
  ----------------------------------- -------------------------------------------------------------------------------------------

### E5. Structured outputs and JSON

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       2 hours

  Topics                              Getting machine-readable, validated output from a probabilistic system

  Subtopics                           Free-text parsing vs JSON mode vs schema-constrained structured outputs;
                                      JSON Schema; Pydantic models as the contract; validation and
                                      retry-with-error-feedback; repair strategies; closed sets (enums) and
                                      explicit unknown/null fields; partial JSON while streaming;
                                      deterministic post-processing; schema versioning; handling refusals and
                                      truncated output

  Learning resources                  OpenAI structured outputs documentation; Anthropic structured output and
                                      tool-use documentation; Pydantic docs

  Practical work                      Build an extraction/classification endpoint (for example support tickets
                                      to JSON) with schema validation, bounded retry on invalid output, and a
                                      measured valid-output rate over 30 samples

  Completion criteria                 Produce validated structured data; handle malformed, truncated and
                                      refused output safely; explain the difference between syntactically
                                      valid JSON and a correct answer

  Dependencies                        E3, E4
  ----------------------------------- ------------------------------------------------------------------------

### E6. Tool calling and MCP (Model Context Protocol)

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Connecting models to code, data and external systems

  Subtopics                           Tool schemas and descriptions; the model proposes, your application
                                      executes; the tool-call loop; parallel tool calls; returning errors to
                                      the model; authorization in application code; idempotent tools; timeouts
                                      and retries; human approval for consequential actions; tool design (few,
                                      clear, token-efficient tools); MCP purpose (a standard protocol so any
                                      AI app can use any tool server); host/client/server roles; primitives
                                      (tools, resources, prompts); JSON-RPC messages; transports (stdio and
                                      streamable HTTP); capability negotiation; authorization basics (OAuth);
                                      MCP security (untrusted servers, tool poisoning, confused deputy, least
                                      privilege); when to use MCP vs plain function calling

  Learning resources                  [Model Context Protocol docs and spec](https://modelcontextprotocol.io);
                                      [MCP Python SDK](https://github.com/modelcontextprotocol/python-sdk);
                                      MCP Inspector; provider function-calling docs; [Anthropic: Building
                                      effective
                                      agents](https://www.anthropic.com/research/building-effective-agents)

  Practical work                      Part 1: hand-write a tool-calling loop with 3 safe tools. Part 2: build
                                      an MCP server exposing 2--3 tools and 1 resource that wrap an existing
                                      REST or Spring Boot service; test with MCP Inspector and connect it to
                                      an MCP-capable client or your own app

  Completion criteria                 Explain the tool-call cycle and the MCP architecture; distinguish model
                                      intent from trusted execution; handle invalid, duplicate and
                                      unauthorized tool calls; justify MCP vs direct function calling for a
                                      given scenario

  Dependencies                        E3, E5; Java/Spring service for the MCP wrapper is a useful reuse
  ----------------------------------- ------------------------------------------------------------------------

### E7. Embeddings, similarity search and vector databases

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Representing meaning as vectors and searching them at scale

  Subtopics                           What an embedding is and what it is not; embedding models, dimensions
                                      and cost; cosine, dot-product and L2 similarity; normalization; semantic
                                      vs lexical search; BM25; exact search vs approximate nearest neighbor
                                      (ANN); HNSW and IVF intuition; recall, latency and memory trade-offs;
                                      vector store options (pgvector vs dedicated stores such as Qdrant or
                                      Pinecone); metadata filtering; indexing, upserts, deletes and
                                      re-embedding when the model changes; hybrid search with rank fusion;
                                      rerankers (cross-encoders); multi-tenant isolation

  Learning resources                  OpenAI embeddings documentation;
                                      [pgvector](https://github.com/pgvector/pgvector); PostgreSQL docs;
                                      Hugging Face MTEB leaderboard for embedding model comparison

  Practical work                      Embed 200--500 chunks, store them in PostgreSQL/pgvector with an HNSW
                                      index, run filtered similarity queries, and compare vector, BM25 and
                                      hybrid retrieval on 15 test queries

  Completion criteria                 Explain why similarity search finds relevant text and when it fails;
                                      justify an index type, distance metric and DB choice; compare retrieval
                                      strategies with measured results

  Dependencies                        E1, E2, SQL basics (HLD D3)
  ----------------------------------- ------------------------------------------------------------------------

### E8. RAG: ingestion, chunking, retrieval and citations

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       6 hours

  Topics                              Grounding model answers in your own knowledge with verifiable sources

  Subtopics                           RAG vs long context vs fine-tuning; the pipeline end to end; parsing
                                      PDF/HTML/Markdown; cleaning; chunking strategies (fixed, recursive,
                                      heading/structure-aware, semantic, parent-child); chunk size and
                                      overlap; metadata; contextual chunk headers; query rewriting and
                                      expansion; top-K and score thresholds; reranking; context assembly under
                                      a token budget; citations (chunk IDs, source spans, quote verification,
                                      provider citation features); refusing when evidence is missing;
                                      freshness, incremental re-indexing and deletion; failure modes (bad
                                      parsing, wrong chunk, lost-in-the-middle, conflicting or stale
                                      documents)

  Learning resources                  [Anthropic: Contextual
                                      Retrieval](https://www.anthropic.com/news/contextual-retrieval);
                                      pgvector docs; LangChain retrieval concepts (use selectively); Chip
                                      Huyen, AI Engineering (RAG chapter)

  Practical work                      Build an ingestion CLI and an /ask endpoint that returns an answer with
                                      citations; run an ablation on chunk sizes (256/512/1024 tokens) and
                                      top-K; programmatically verify that each cited chunk supports the claim

  Completion criteria                 Explain the RAG pipeline and where each failure mode originates; build a
                                      working ingestion-to-cited-answer path; measure retrieval quality and
                                      citation accuracy on a test set

  Dependencies                        E3, E4, E5, E7
  ----------------------------------- ------------------------------------------------------------------------

### E9. Evals: testing and measuring AI systems

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Engineering discipline for non-deterministic software

  Subtopics                           Why a demo is not an evaluation; golden datasets (start with 30--50
                                      cases, grow to 100+); reading traces and error analysis; code-based
                                      checks vs LLM-as-judge; judge biases (position, verbosity) and
                                      calibration against human labels; retrieval metrics (recall@k,
                                      precision@k, MRR, nDCG); generation metrics (faithfulness/groundedness,
                                      answer relevance, citation accuracy); structured-output validity;
                                      tool-call correctness; agent outcome and trajectory evals; run-to-run
                                      variance; regression suites in CI; online feedback; tracing tools
                                      (Langfuse, LangSmith, Phoenix)

  Learning resources                  [Hamel Husain: Your AI Product Needs
                                      Evals](https://hamel.dev/blog/posts/evals/); [Langfuse
                                      docs](https://langfuse.com/docs); provider evaluation guides; Chip
                                      Huyen, AI Engineering (evaluation chapters)

  Practical work                      Build an eval harness (pytest or script) for the RAG project with
                                      retrieval and answer metrics; measure one change before and after;
                                      calibrate an LLM judge against 20 hand-labeled cases

  Completion criteria                 Report measurable quality before and after a change; identify false
                                      positives and false negatives; explain limits of model-as-judge; run
                                      evals as a regression gate

  Dependencies                        E5, E8; E6 for tool-use evals
  ----------------------------------- ------------------------------------------------------------------------

### E10. Agents, workflows, memory and LangGraph

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       6 hours

  Topics                              Multi-step AI systems with bounded autonomy

  Subtopics                           Workflow vs agent; workflow patterns (prompt chaining, routing,
                                      parallelization, orchestrator-workers, evaluator-optimizer); the agent
                                      loop and ReAct-style reasoning; planning; stop conditions, step limits
                                      and budgets; context management in long-running agents (compaction,
                                      scratch notes, sub-agents); memory (short-term thread state; long-term
                                      episodic, semantic and procedural memory; summarization; retrieval-based
                                      memory; user profile memory; privacy and deletion); LangGraph (state
                                      schema, nodes, edges, conditional edges, reducers, checkpointers and
                                      threads, interrupts for human-in-the-loop, streaming); multi-agent
                                      designs and when not to use them; failure modes (loops, compounding
                                      errors, tool misuse, cost blow-up); awareness of vendor agent SDKs

  Learning resources                  [Anthropic: Building effective
                                      agents](https://www.anthropic.com/research/building-effective-agents);
                                      [LangGraph docs](https://docs.langchain.com/oss/python/langgraph/);
                                      [Anthropic: Writing tools for
                                      agents](https://www.anthropic.com/engineering/writing-tools-for-agents)

  Practical work                      Hand-write an agent loop (about 100 lines) with a step limit and budget;
                                      then reimplement the same workflow in LangGraph with checkpointing,
                                      memory and a human approval step; compare the two honestly

  Completion criteria                 Explain state transitions, recovery and memory design; justify when a
                                      workflow beats an agent and when a framework is warranted; prevent
                                      unbounded loops and uncontrolled tool execution

  Dependencies                        E6, E8, E9
  ----------------------------------- ------------------------------------------------------------------------

### E11. Guardrails, security and privacy

  ----------------------------------- -------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------------------------------------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Secure AI application design

  Subtopics                           Direct and indirect prompt injection (retrieved documents, tool outputs, MCP servers); data
                                      exfiltration paths; excessive agency and least privilege; authorization at the tool
                                      boundary; tenant isolation in retrieval; PII handling and redaction; secrets; output
                                      validation; hallucination controls (grounding checks, citation verification, abstention);
                                      moderation and safety policies; audit logging; retention and deletion; human approval for
                                      consequential actions; OWASP Top 10 for LLM Applications

  Learning resources                  [OWASP Top 10 for LLM
                                      Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/);
                                      provider security guidance; MCP security best practices

  Practical work                      Create a red-team set of 15 attacks (injected documents, jailbreak attempts, cross-tenant
                                      queries, unauthorized tool calls); enforce authorization outside the model; add the cases
                                      to the eval suite

  Completion criteria                 Explain trust boundaries; demonstrate authorization outside the model; pass
                                      prompt-injection and data-isolation tests

  Dependencies                        E6--E10
  ----------------------------------- -------------------------------------------------------------------------------------------

### E12. Production operations: cost, latency, reliability and observability

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       2 hours

  Topics                              Operating AI features as real services

  Subtopics                           Per-request token, cost and latency tracking; prompt caching; semantic
                                      caching and its risks; model routing (small vs large models); batch
                                      APIs; async processing of long tasks with queues; streaming UX;
                                      timeouts, retries, circuit breakers and fallbacks; rate and concurrency
                                      limits; tracing with OpenTelemetry; request IDs; prompt and model
                                      versioning; provider outages and graceful degradation; quality SLOs
                                      alongside latency SLOs

  Learning resources                  Provider production guides; [OpenTelemetry
                                      docs](https://opentelemetry.io/docs/); HLD tracks D5, D7 and D8

  Practical work                      Add per-request cost/latency logging and a simple dashboard or report;
                                      simulate a provider timeout; implement a fallback and a per-request
                                      budget

  Completion criteria                 Explain the cost and latency of a request; diagnose a failed call from
                                      traces; demonstrate bounded retries, timeouts and useful telemetry

  Dependencies                        E3, E8, E9, E10
  ----------------------------------- ------------------------------------------------------------------------

### E13. Working effectively with AI coding agents

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            Medium

  Time estimate                       2 hours

  Topics                              Using AI tools to multiply, not replace, engineering judgment

  Subtopics                           Delegating well-scoped tasks; project context files (for example
                                      CLAUDE.md or AGENTS.md); plan first, then implement; small reviewable
                                      diffs; tests as the verification loop; reviewing AI-generated code for
                                      correctness, security and maintainability; when to hand-write instead
                                      (learning tasks, core design decisions); MCP-connected developer tools;
                                      cost and context hygiene

  Learning resources                  Claude Code documentation; GitHub Copilot or Cursor documentation
                                      (choose one); your own failure log

  Practical work                      Use a coding agent to add one feature with tests to the capstone; review
                                      the diff line by line; record where the agent erred and how tests or
                                      review caught it

  Completion criteria                 Deliver an AI-assisted change that you can fully explain and defend;
                                      show a repeatable workflow with context file, plan, tests and review

  Dependencies                        E1; applies throughout. Rule 6 still holds for learning tasks
  ----------------------------------- ------------------------------------------------------------------------

### E14. Capstone: build one real AI application

  ----------------------------------- ------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ------------------------------------------------------------------------
  Priority                            High

  Time estimate                       8--10 hours across the six months

  Topics                              End-to-end applied AI engineering

  Subtopics                           Pick one real workflow and user; define success metrics; build a FastAPI
                                      service; integrate a model with streaming; structured outputs; add RAG
                                      with citations over a real corpus; expose capabilities through an MCP
                                      server; add a bounded workflow or agent only where needed; persist state
                                      and memory; authentication and tenant scoping if applicable; evaluation
                                      suite; observability; documentation of deployment and limitations

  Learning resources                  E1--E13 resources; provider documentation is the source of truth for API
                                      behavior

  Practical work                      Suggested: an engineering knowledge or runbook assistant, a support
                                      copilot, or a study assistant over your own notes. Suggested stack:
                                      Python/FastAPI, PostgreSQL/pgvector, Docker, MCP server, LangGraph only
                                      if a graph is justified, simple UI, Spring Boot integration if useful

  Completion criteria                 A locally reproducible or deployed app; README and architecture diagram;
                                      eval dataset and report; cost/latency report; security notes; demo of
                                      failure handling; ability to explain every design decision in an
                                      interview

  Dependencies                        E1--E13 progressively
  ----------------------------------- ------------------------------------------------------------------------

### Capstone milestones

  -----------------------------------------------------------------------------
  Milestone               Target phase            Deliverable
  ----------------------- ----------------------- -----------------------------
  M1 --- Problem          Weeks 1--4              One-page problem statement,
  definition                                      user journey, success metric,
                                                  architecture sketch

  M2 --- Model            Weeks 5--8              Streaming API call,
  integration and                                 versioned system prompt,
  structured output                               validated JSON output, logs

  M3 --- Tools and MCP    Weeks 9--10             Tool-calling loop and an MCP
                                                  server wrapping one real
                                                  service

  M4 --- Retrieval        Weeks 11--15            Ingestion, pgvector search,
  and RAG                                         hybrid retrieval, cited
                                                  answers

  M5 --- Evals and        Weeks 16--20            Golden dataset, regression
  agentic workflow                                evals, bounded workflow or
                                                  agent with memory and
                                                  human approval

  M6 --- Hardening        Weeks 21--26            Security tests, cost/latency
  and walkthrough                                 controls, README, demo,
                                                  trade-offs and known
                                                  limitations
  -----------------------------------------------------------------------------

**Mentor warning:** Do not start with a multi-agent system. Start with a
direct model call, then structured output, then tools and retrieval,
then a deterministic workflow. Introduce an agent or framework only when
dynamic planning is genuinely required and you can show it beats the
simpler design on your evals.

### AI interview self-test question bank

Answer each aloud in 3--5 minutes without notes, with trade-offs.

-   Walk through what happens from a prompt string to a generated
    response. Where do tokens, the context window and sampling fit?
-   Why do LLMs hallucinate, and which five techniques reduce it?
-   Design a RAG system over 10 million documents: ingestion, chunking,
    index, retrieval, reranking, citations, freshness, access control.
-   How do you pick chunk size and top-K, and how do you prove the
    choice is right?
-   Compare pgvector with a dedicated vector database for a production
    workload.
-   How do you guarantee valid JSON from a model? What can still go
    wrong?
-   Explain MCP. When would you use it instead of plain function
    calling?
-   Design an agent that can take consequential actions. Where do
    authorization, approval, limits and rollback live?
-   Workflow or agent: how do you decide?
-   How do you give an assistant memory across sessions without leaking
    data between users?
-   How do you evaluate a RAG system and an agent? What are the
    limitations of LLM-as-judge?
-   A conversation exceeds the context window. What are your options?
-   Estimate and reduce the cost and latency of an LLM feature at 1
    million requests per day.
-   Describe a prompt injection attack through a retrieved document and
    how your design contains it.

------------------------------------------------------------------------

# 8. Cross-track dependency map

The same concept appears in multiple tracks. The application should link
related topics rather than schedule redundant learning.

  -----------------------------------------------------------------------
  Foundation              Enables                 Cross-track connection
  ----------------------- ----------------------- -----------------------
  Java object model and   LLD and Java interviews Collections underpin
  collections                                     graph/heap DSA
                                                  implementations and LLD
                                                  domain models

  Recursion and trees     DSA, LLD reasoning      Backtracking and tree
                                                  traversal support
                                                  recursive object
                                                  structures

  Java concurrency and    LLD, HLD and production Thread safety, worker
  memory model            engineering             pools, shared state and
                                                  backpressure

  SQL and transactions    Java, HLD and AI        Persistence, isolation,
                                                  idempotency and vector
                                                  metadata

  Networking and APIs     HLD and AI              Model APIs,
                                                  service-to-service
                                                  communication, timeouts
                                                  and retries

  Queues and event-driven HLD, Java and AI       Async ingestion,
  design                                          background jobs and
                                                  tool execution

  Testing and             All tracks              Evidence that
  observability                                   code/design works and
                                                  can be operated

  SOLID and design        LLD and Java            Extensible
  patterns                                        integrations, provider
                                                  abstractions and clean
                                                  service boundaries

  Distributed systems     HLD and AI              Failure handling,
  fundamentals                                    state, retries,
                                                  consistency and
                                                  resilience
  -----------------------------------------------------------------------

------------------------------------------------------------------------

# 9. Weekly roadmap: 26-week sequence

The weekly plan identifies the primary focus, not every task. Continue
the NeetCode official sequence from the first incomplete problem
throughout all weeks. Use the daily scheduler to interleave revisions
and the other tracks.

  -----------------------------------------------------------------------------------------------------------------------------------------
            Week Java                 DSA --- official    LLD --- CodeArmy            HLD                      AI Upskilling
                                      NeetCode 250
  -------------- -------------------- ------------------- --------------------------- ------------------------ ----------------------------
               1 A1 language/object   Resume first        C1 approach and OOP         D1 requirements and      E1 Python for Java devs
                 model baseline       incomplete official                             estimation               (uv, typing)
                                      category; baseline
                                      1--2 problems

               2 A2 collections       Continue official   C1 UML and modeling         D2 request lifecycle     E1 Pydantic, async,
                 foundations          order; arrays/hash                                                       FastAPI, pytest
                                      and/or next
                                      incomplete topic

               3 A2 HashMap internals Continue official   C2 SOLID                    D3 SQL/data modeling     E2 tokens, tokenizers,
                 and comparators      order; two                                                               next-token prediction
                                      pointers/sliding
                                      window as
                                      applicable

               4 A3 streams and       Continue official   C2 refactoring exercise     D3 indexes/transactions  E2 context window,
                 modern Java          order; binary                                                            sampling, limits,
                                      search/linked list                                                       hallucinations
                                      as applicable

               5 A4 threads and race  Continue official   C3                          D4 caching               E3 first LLM API app,
                 conditions           order; linked       factory/builder/singleton                            streaming, token usage
                                      list/stack as
                                      applicable

               6 A4 synchronization   Continue official   C3 implement patterns       D4 cache invalidation    E3 retries, rate limits,
                 and JMM              order; stack/tree                                                        context budgeting
                                      as applicable

               7 A5 locks,            Continue official   C4 adapter/decorator/facade D5 queues and Kafka      E4 system prompts,
                 conditions,          order; trees/tries                              concepts                 context engineering
                 executors            as applicable

               8 A5 CompletableFuture Continue official   C4 proxy/composite          D5 retries, DLQ, outbox  E5 JSON / structured
                 and virtual threads  order;                                                                   output + validation
                                      backtracking/heap
                                      as applicable

               9 A6 JVM memory and    Continue official   C5 strategy/observer        D6 consistency and       E6 tool-calling loop,
                 class loading        order; graph                                    replication              authz, idempotency
                                      foundations as
                                      applicable

              10 A6 GC and memory     Continue official   C5 command/state/chain      D6 partial failures      E6 build an MCP server,
                 troubleshooting      order; graphs                                                            connect a client

              11 A7 JFR and profiling Continue official   C6 first independent        D7 scaling and           E7 embeddings,
                                      order; advanced     project                     availability             similarity search
                                      graphs

              12 A8 Spring DI and     Continue official   C6 second independent       D7 rate                  E7 vector DBs, pgvector,
                 bean lifecycle       order; 1-D DP       project                     limiting/resilience      ANN, hybrid search

              13 A8 proxies and       Continue official   C6 third project            D8                       E8 ingestion, parsing,
                 transactions         order; 1-D DP                                   observability/security   chunking, metadata

              14 A9 SQL               Continue official   C6 fourth project           D8 SLOs and tracing      E8 retrieval, reranking,
                 isolation/indexing   order; 2-D DP                                                            citations

              15 A9 idempotency and   Continue official   C7 testing and concurrency  D9 URL                   E8 RAG failure modes;
                 locking              order; 2-D DP                                   shortener/notification   vertical slice done
                                                                                      design

              16 A10 tests and        Continue official   C7 timed machine coding     D9 feed/chat design      E9 golden dataset,
                 production concerns  order;                                                                   error analysis
                                      greedy/intervals

              17 Java revision:       Continue official   Rebuild weakest LLD project D9 payment/order design  E9 RAG evals, LLM judge,
                 collections +        order; remaining                                                         regression runs
                 concurrency          greedy/intervals

              18 Java revision: JVM + Continue official   Pattern selection drills    D9 search/scheduler      E10 workflow patterns,
                 Spring               order; bit                                      design                   chaining and routing
                                      manipulation

              19 Java mock interview  Close remaining     60--90 min machine coding   D10 architecture         E10 agent loop, memory,
                                      official DSA                                    trade-offs               bounded autonomy
                                      categories as
                                      feasible

              20 Weak-area            Timed NeetCode      LLD mock 1                  HLD mock 1               E10 LangGraph state,
                 remediation          re-solves                                                                checkpoints, HITL

              21 Java concurrency     Timed NeetCode      LLD mock 2                  HLD mock 2               E11 prompt injection,
                 mock                 re-solves                                                                isolation, tool authz

              22 Spring/JVM deep      Weak-problem        LLD mock 3                  HLD mock 3               E12 cost, latency,
                 revision             revision                                                                 caching, tracing

              23 Java mixed interview Mixed timed set     Explain project design      Design unfamiliar system E13 AI-assisted dev;
                 set                  from NeetCode only                                                       capstone hardening

              24 Java flash recall +  Re-solve weak       Rebuild one project from    Revise weak design areas E14 fix weakest eval
                 coding               high-priority       blank                                                cases; agent evals
                                      problems

              25 Full Java mock       Timed DSA mock      Timed LLD mock              Timed HLD mock           E14 failure drill;
                                                                                                               AI mock interview

              26 Final interview      Final NeetCode 250  Final design review         Final design review      E14 final demo and
                 consolidation        status + weak list                                                       architecture review
  -----------------------------------------------------------------------------------------------------------------------------------------

### How to interpret the DSA column

The weekly DSA column is intentionally pattern-oriented, not a
replacement problem list. If your current NeetCode state is already
beyond a named category, do not go backwards. Continue with the next
incomplete problem in the official sheet. The app should calculate
actual category progress from the imported sheet data.

------------------------------------------------------------------------

# 10. Daily agenda and revision engine requirements

This section is intended for the application that will schedule the
work.

## 10.1 Daily agenda composition

A normal study day should contain:

-   **One primary task:** a new topic, problem, design exercise or
    project milestone.
-   **One revision task:** due from the revision queue.
-   **One small output:** code, diagram, explanation, test, or written
    decision.
-   **Optional stretch task:** only when the planned work is complete
    and energy/time remain.

Do not schedule five tracks every day. A day should normally have one or
two tracks.

## 10.2 Suggested task duration

  Task type                          Default duration
  -------------------------------- ------------------
  Java concept + code experiment           45--60 min
  One NeetCode problem                     35--60 min
  NeetCode revision                        15--30 min
  LLD lecture + notes                      30--45 min
  LLD implementation                       45--90 min
  HLD concept                              30--45 min
  HLD design exercise                      45--60 min
  AI concept + implementation              45--90 min
  Weekly review                                30 min

## 10.3 Revision intervals

Use 1, 3, 7, 14, 30 and 60-day intervals as defaults. The interval
should depend on confidence and recall quality.

  -----------------------------------------------------------------------
  Recall result                       Action
  ----------------------------------- -----------------------------------
  Correct, clear, no notes            Move to next interval

  Correct but slow/uncertain          Repeat at the same or next shorter
                                      interval

  Partially correct                   Schedule in 1--3 days

  Incorrect / cannot explain          Return to learning state and
                                      schedule in 1 day

  Repeatedly failed                   Break into smaller subtopics and
                                      add a practical exercise
  -----------------------------------------------------------------------

For conceptual topics, use active recall questions. For code topics, ask
the learner to reproduce a small implementation. For design topics, ask
for a diagram and trade-off explanation. For DSA, re-solve the exact
official problem.

## 10.4 Priority and due-date scoring

A practical initial scoring model for the scheduler:

`task_score = priority_weight + overdue_weight + weakness_weight + dependency_weight + track_balance_weight`

Suggested weights: - High priority: 5; Medium: 3; Low: 1. - Overdue
revision: add 1--4 depending on lateness. - Weak recall / failed
previous attempt: add 3. - Unblocks dependent topics: add 2. - Track
balance: add 1 when a track has been neglected for too long.

This is a starting heuristic. Avoid letting a numerical score override
prerequisites or daily capacity.

## 10.5 Daily capacity rules

-   Default planned study time: 45--75 minutes on weekdays; 2--2.5 hours
    on one weekend day.
-   Never fill 100% of the available time. Keep 10--20% buffer.
-   If the user marks a day unavailable, move tasks without duplicating
    them.
-   If a task is missed, reschedule it based on priority and
    dependency---not automatically to tomorrow.
-   Limit new heavy concepts to one per day.
-   Keep revision tasks shorter than new-learning tasks.
-   Preserve at least one low-intensity/rest day per week.

## 10.6 Definition of a successful study day

A day is successful when the planned learning output is produced---not
when a timer reaches a target.

Examples of valid outputs: - A problem solved and explained. - A concept
recalled and code experiment completed. - A class diagram plus working
implementation. - An architecture diagram plus trade-off notes. - A
working AI feature plus test/evaluation evidence.

------------------------------------------------------------------------

# 11. Progress metrics and six-month success criteria

Track outcome quality, not only time spent.

  -----------------------------------------------------------------------
  Track                   Leading metric          Six-month success
                                                  evidence
  ----------------------- ----------------------- -----------------------
  Java                    Concepts explained from Strong command of
                          memory; code            collections,
                          experiments; mock       concurrency, JVM,
                          performance             Spring and persistence
                                                  fundamentals

  DSA                     Official problems       All or most remaining
                          solved, interview-ready NeetCode 250 problems
                          count, revision         completed with weak
                          retention               problems identified and
                                                  revised; no external
                                                  DSA additions

  LLD                     Independent             4+ substantial projects
                          implementations and     independently
                          timed exercises         implemented; 3 timed
                                                  mocks

  HLD                     Designs completed and   5--6 complete system
                          trade-offs defended     designs and 3 timed
                                                  mocks

  AI upskilling           Working increments,     One coherent AI-enabled
                          evaluation quality,     application with tests,
                          reliability             evaluation, security
                                                  considerations,
                                                  telemetry and
                                                  reproducible setup

  Consistency             Planned sessions        Sustainable 7--8 hour
                          completed / weekly      weekly average without
                          review                  sacrificing sleep,
                                                  health or work
  -----------------------------------------------------------------------

### Weekly review questions

1.  What did I independently learn or implement this week?
2.  Which topic can I explain without notes?
3.  Which topic did I only recognize while reading, but cannot
    reproduce?
4.  Which DSA problems failed during revision, and why?
5.  What did I ship in the AI capstone, and what did my evals show?
6.  What is the single biggest blocker for next week?
7.  Is the current weekly workload sustainable?

### Monthly checkpoint

At the end of each four-week block: - Take one Java mock or oral
question session. - Take one timed DSA session using only NeetCode 250
problems. - Complete one LLD or HLD design exercise. - Demonstrate one
tangible AI capstone improvement. - Remove or defer low-value topics if
the workload is not sustainable. - Begin/continue targeted job
applications; do not wait for the roadmap to be fully complete.

------------------------------------------------------------------------

# 12. Recommended resource library

Use a small number of high-quality sources. Do not turn resource
collection into another project.

## Java

-   [Dev.java learning hub](https://dev.java/learn/) --- official
    tutorials for language, collections, streams, virtual threads and
    JVM tools.
-   [Java SE specifications](https://docs.oracle.com/en/java/javase/)
    --- authoritative language and JVM behavior.
-   [Spring Framework
    Reference](https://docs.spring.io/spring-framework/reference/) ---
    DI, bean lifecycle, AOP and transactions.
-   *Effective Java* --- Joshua Bloch.
-   *Java Concurrency in Practice* --- Brian Goetz et al. (classic
    conceptual reference; supplement with current JDK docs).

## DSA

-   [Official NeetCode
    250](https://neetcode.io/practice/practice/neetcode250) --- sole
    source of assigned problems.
-   NeetCode's own explanations for problems in the sheet.
-   Java API documentation when implementation details are unclear.

## LLD

-   [Coder Army LLD
    playlist](https://youtube.com/playlist?list=PLQEaRBV9gAFvzp6XhcNFpk1WdOcyVo9qT).
-   [Coder Army companion GitHub
    repository](https://github.com/adityatandon15/Low-Level-Design-Course).
-   [Refactoring.Guru design
    patterns](https://refactoring.guru/design-patterns) --- reference
    after attempting a pattern.
-   *Head First Design Patterns* --- optional reference for intuition.

## HLD

-   [System Design
    Primer](https://github.com/donnemartin/system-design-primer) ---
    broad topic map and interview examples.
-   *Designing Data-Intensive Applications* --- Martin Kleppmann; use
    selected chapters for distributed systems depth.
-   *System Design Interview* --- Alex Xu; use as case-study practice,
    not as a memorization source.
-   [AWS Well-Architected
    Framework](https://docs.aws.amazon.com/wellarchitected/latest/userguide/waf.html)
    --- reliability, security, operations, performance and cost
    trade-offs.
-   [Google SRE books](https://sre.google/books/) --- operational
    excellence, SLOs and reliability.

## AI Engineering

-   [OpenAI developer
    documentation](https://developers.openai.com/api/docs/) --- APIs,
    tools, structured outputs and embeddings.
-   [Anthropic documentation](https://docs.claude.com) --- Claude API,
    prompting, tool use and citations.
-   [Anthropic --- Building effective
    agents](https://www.anthropic.com/research/building-effective-agents)
    --- workflow/agent distinction and simplicity-first design.
-   [Anthropic --- Effective context engineering for
    agents](https://www.anthropic.com/engineering/effective-context-engineering-for-ai-agents).
-   [Model Context Protocol](https://modelcontextprotocol.io) --- spec,
    SDKs and server/client guides.
-   [LangGraph
    documentation](https://docs.langchain.com/oss/python/langgraph/) ---
    stateful workflows, persistence and orchestration.
-   [Hugging Face LLM Course](https://huggingface.co/learn/llm-course)
    --- tokenizers and transformer fundamentals.
-   Andrej Karpathy's *Intro to Large Language Models* and tokenizer
    videos; 3Blue1Brown transformer videos.
-   *AI Engineering* --- Chip Huyen; broad applied reference for
    foundation models, RAG, evals and production.
-   [Hamel Husain --- Your AI Product Needs
    Evals](https://hamel.dev/blog/posts/evals/).
-   [FastAPI tutorial](https://fastapi.tiangolo.com/tutorial/) ---
    Python API service.
-   [pgvector](https://github.com/pgvector/pgvector) --- vector
    similarity search in PostgreSQL.
-   [OWASP Top 10 for LLM
    Applications](https://owasp.org/www-project-top-10-for-large-language-model-applications/)
    --- AI application security.
-   [OpenTelemetry](https://opentelemetry.io/docs/) --- traces, metrics
    and logs.

------------------------------------------------------------------------

# 13. Mentor's final guidance

The objective is not to become equally advanced in five domains in six
months. The objective is to build a credible, interview-ready profile:

> **A strong backend engineer who understands Java and distributed
> systems deeply, can solve algorithmic problems reliably, can design
> maintainable services, and understands how LLMs work, and can
> build, evaluate and secure production-minded AI applications.**

Three rules matter most:

1.  **Depth over content consumption.** One topic that you can explain,
    implement and debug is worth more than five videos watched
    passively.
2.  **Build evidence.** Each week should leave behind code, a solved
    official problem, a design diagram, a test, or a written
    explanation.
3.  **Use AI deliberately.** For learning tracks, ask it to mentor,
    question and review. For utility work, delegation is fine---but
    understand the architecture, security, data and operational
    consequences of what you ship.

A six-month roadmap is a guide, not a contract. If work becomes intense,
reduce the number of new topics and protect revision. Consistency and
retained understanding are the success criteria.

------------------------------------------------------------------------

## Research and source notes

This roadmap was prepared using the official NeetCode 250 page, the
Coder Army LLD playlist and companion repository, Java/Dev.java
documentation, the Java specifications, Spring documentation, System
Design Primer, AWS Well-Architected guidance, OpenAI developer
documentation, Anthropic's agent and context engineering guidance,
Model Context Protocol documentation, LangGraph documentation and
pgvector documentation.

-   NeetCode 250 official counts and topic groups:
    https://neetcode.io/practice/practice/neetcode250
-   Coder Army LLD course code and playlist reference:
    https://github.com/adityatandon15/Low-Level-Design-Course
-   System Design Primer:
    https://github.com/donnemartin/system-design-primer
-   Anthropic agent guidance:
    https://www.anthropic.com/research/building-effective-agents
-   Model Context Protocol: https://modelcontextprotocol.io
-   Java learning resources: https://dev.java/learn/
-   AWS Well-Architected:
    https://docs.aws.amazon.com/wellarchitected/latest/userguide/waf.html
-   OpenAI API documentation: https://developers.openai.com/api/docs/
-   pgvector: https://github.com/pgvector/pgvector
