# Six-Month Engineering Interview & FDE Roadmap

**Planning horizon:** 26 weeks (approximately six months)\
**Tracks:** Java Interview Preparation · NeetCode 250 · Low-Level Design
· High-Level Design · Forward Deployed / AI Engineering\
**Intended learner:** Backend engineer with professional Java/Spring
Boot experience, preparing for stronger SDE interviews while building
practical AI application engineering capability.\
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
5.  **FDE is not model research.** Focus on shipping reliable software
    using existing models and APIs, not training a foundation model from
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

  FDE / AI engineering                  1.5--2 hours Continuous
                                                     project-based
                                                     learning and shipping
  ------------------------------------------------------------------------

This is a baseline, not a rigid quota. During an FDE project milestone,
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

  Saturday         FDE project +             2--2.5 hours A working project
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
                                                     fundamentals, and LLM API
                                                     basics

  Phase 2 --- Core                              5--9 Collections/concurrency,
  interview depth                                    trees/graphs/DP progress,
                                                     design patterns,
                                                     databases/caching,
                                                     structured LLM workflows

  Phase 3 --- Applied                         10--14 Java performance and
  engineering                                        Spring internals, advanced
                                                     DSA, LLD projects,
                                                     distributed systems, RAG
                                                     vertical slice

  Phase 4 ---                                 15--19 Timed DSA,
  Production readiness                               concurrency/testing, HLD
                                                     case studies, agent/tool
                                                     workflows, evals and
                                                     observability

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

# 7. Track E --- Forward Deployed / AI Application Engineering (FDE)

## Track objective

Become a backend engineer who can build, integrate, deploy and operate
AI-powered applications for real users. The focus is applied
engineering: connect models to business data and tools, evaluate
behavior, secure the system, and make it reliable and cost-aware.

**Not the goal:** training foundation models, deep mathematical ML
research, or collecting agent frameworks without building working
systems.

**Estimated total:** 45--55 focused hours across six months, including
one coherent project.\
**Suggested stack:** Python fundamentals + one model API + Spring Boot
or FastAPI service boundary + PostgreSQL/pgvector + Docker. Choose one
orchestration framework only after understanding direct API/tool calls.
Do not learn every vendor/framework simultaneously.

### E1. Python for AI application engineering

  ----------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ----------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Practical Python needed to build AI services

  Subtopics                           Types and data structures; functions; classes;
                                      modules; exceptions; iterators/generators;
                                      comprehensions; virtual environments; package
                                      management; typing; Pydantic models; async/await;
                                      HTTP clients; FastAPI basics; pytest

  Learning resources                  [Python official
                                      tutorial](https://docs.python.org/3/tutorial/);
                                      [FastAPI
                                      tutorial](https://fastapi.tiangolo.com/tutorial/);
                                      [Pydantic documentation](https://docs.pydantic.dev/)

  Practical work                      Build a typed FastAPI endpoint with validation,
                                      async HTTP call, error handling and tests

  Completion criteria                 Build and test a small API without relying on
                                      generated code for basic Python syntax or debugging

  Dependencies                        None; Java experience helps with OOP concepts
  ----------------------------------------------------------------------------------------

### E2. LLM foundations and model API integration

  ----------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ----------------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              How to integrate foundation models into applications

  Subtopics                           Tokens and tokenization; context windows;
                                      system/developer/user instructions; sampling basics; model
                                      selection; API requests/responses; streaming; timeouts;
                                      retries; rate limits; token usage; cost estimation; prompt
                                      versioning; provider abstraction; secrets management

  Learning resources                  [OpenAI API
                                      documentation](https://developers.openai.com/api/docs/);
                                      provider documentation for the chosen model; OpenAI
                                      Cookbook

  Practical work                      Build a small model gateway with streaming, usage logging,
                                      timeout/retry behavior and a configurable provider/model

  Completion criteria                 Explain request lifecycle, context/token constraints,
                                      streaming and cost drivers; integrate an LLM API safely

  Dependencies                        E1
  ----------------------------------------------------------------------------------------------

### E3. Prompting, structured outputs and context engineering

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       3 hours

  Topics                              Reliable model input and output

  Subtopics                           Prompt structure; task
                                      decomposition; examples/few-shot
                                      prompting; structured JSON outputs;
                                      schema validation; output parsing;
                                      context selection; context
                                      compression; prompt injection
                                      awareness; deterministic
                                      post-processing; prompt/version
                                      management

  Learning resources                  OpenAI structured outputs
                                      documentation; Anthropic prompting
                                      and context engineering guides

  Practical work                      Create a structured
                                      extraction/classification endpoint
                                      with schema validation and
                                      malformed-output handling

  Completion criteria                 Produce validated structured data,
                                      handle invalid output safely and
                                      explain why prompting alone is not
                                      a correctness guarantee

  Dependencies                        E2
  -----------------------------------------------------------------------

### E4. Tool/function calling and workflow design

  ---------------------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- ---------------------------------------------------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Connecting models to application capabilities

  Subtopics                           Tool schemas; tool selection; validation; authorization; execution loop;
                                      tool result handling; idempotent tools; timeouts; retries; human approval;
                                      workflow vs agent distinction; sequential, parallel and conditional
                                      workflows

  Learning resources                  [OpenAI function
                                      calling](https://developers.openai.com/api/docs/guides/function-calling);
                                      [Anthropic --- Building effective
                                      agents](https://www.anthropic.com/research/building-effective-agents)

  Practical work                      Build an assistant with 3--4 safe tools (search records, create task,
                                      summarize item, retrieve status); enforce authorization in application code

  Completion criteria                 Explain the tool-call cycle; distinguish model intent from trusted
                                      application execution; handle invalid, duplicate and unauthorized tool
                                      calls

  Dependencies                        E2, E3
  ---------------------------------------------------------------------------------------------------------------

### E5. Embeddings and retrieval-augmented generation (RAG)

  -------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------------------------
  Priority                            High

  Time estimate                       6 hours

  Topics                              Grounding model responses in external knowledge

  Subtopics                           Embeddings; semantic similarity; cosine/dot-product
                                      distance; document ingestion; parsing; chunking;
                                      overlap; metadata; vector stores; PostgreSQL/pgvector;
                                      top-K retrieval; metadata filtering; hybrid search;
                                      reranking; citations; retrieval evaluation; freshness
                                      and deletion

  Learning resources                  OpenAI embeddings documentation; [pgvector
                                      documentation](https://github.com/pgvector/pgvector);
                                      PostgreSQL docs; LangChain retrieval concepts (use
                                      selectively)

  Practical work                      Ingest PDFs/Markdown, chunk and embed them, store
                                      metadata/vectors, retrieve relevant chunks, generate
                                      cited answers; compare chunk sizes and top-K settings

  Completion criteria                 Explain the RAG pipeline and failure modes; build a
                                      working ingestion/query path; measure retrieval quality
                                      on a small test set

  Dependencies                        E1, E2, E3, SQL basics
  -------------------------------------------------------------------------------------------

### E6. Agent orchestration and state management

  -------------------------------------------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -------------------------------------------------------------------
  Priority                            Medium

  Time estimate                       4 hours

  Topics                              Multi-step AI workflows

  Subtopics                           Workflow graphs; state; routing; conditional edges; retries;
                                      checkpointing; short-term vs long-term memory; human-in-the-loop;
                                      interrupt/resume; tool boundaries; agent loop; bounded autonomy;
                                      termination criteria

  Learning resources                  [LangGraph
                                      documentation](https://docs.langchain.com/oss/python/langgraph/);
                                      Anthropic --- Building effective agents

  Practical work                      Build a bounded research/task workflow with explicit states, retry
                                      limits and a human approval step; compare direct function
                                      orchestration with a framework

  Completion criteria                 Explain state transitions and recovery; demonstrate why a
                                      graph/framework is justified; prevent unbounded loops and
                                      uncontrolled tool execution

  Dependencies                        E4, E5
  -------------------------------------------------------------------------------------------------------

### E7. Evaluation, testing and quality measurement

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       5 hours

  Topics                              Testing probabilistic software

  Subtopics                           Golden datasets; test cases;
                                      exact-match vs semantic metrics;
                                      retrieval precision/recall; answer
                                      relevance;
                                      faithfulness/groundedness;
                                      tool-call correctness; regression
                                      evaluation; model-as-judge
                                      limitations; human review; online
                                      feedback; experiment tracking

  Learning resources                  OpenAI Evals documentation; RAG
                                      evaluation material from the chosen
                                      framework; Anthropic evaluation
                                      guidance

  Practical work                      Create 30--50 representative test
                                      questions; record expected source
                                      documents and acceptable outcomes;
                                      run prompt/retrieval changes
                                      against the same dataset

  Completion criteria                 Report measurable quality
                                      before/after a change; identify
                                      false positives and false
                                      negatives; explain why a demo is
                                      not an evaluation

  Dependencies                        E3, E5; E4 for tool-use evaluations
  -----------------------------------------------------------------------

### E8. Guardrails, security and privacy

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Secure AI application design

  Subtopics                           Prompt injection; untrusted
                                      retrieved content; data leakage;
                                      tenant isolation; PII handling;
                                      authorization at tool boundary;
                                      least privilege; secrets; output
                                      validation; moderation/safety
                                      policies; auditability;
                                      retention/deletion; human approval
                                      for consequential actions

  Learning resources                  OWASP Top 10 for LLM Applications;
                                      provider security documentation;
                                      application security guidance

  Practical work                      Add adversarial documents and
                                      prompts to test set; ensure
                                      retrieval respects user/tenant
                                      scope; prevent a model from
                                      invoking unauthorized tools

  Completion criteria                 Explain trust boundaries;
                                      demonstrate authorization outside
                                      the model; test prompt injection
                                      and data isolation scenarios

  Dependencies                        E2--E6
  -----------------------------------------------------------------------

### E9. Production operations: latency, cost, reliability and observability

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       4 hours

  Topics                              Operating an AI feature in
                                      production

  Subtopics                           Token/cost tracking; prompt caching
                                      concepts; streaming UX; timeouts;
                                      retries; rate limits; concurrency
                                      limits; fallbacks; model routing;
                                      batching; caching; tracing; request
                                      IDs; prompt/model versioning;
                                      provider outages; graceful
                                      degradation

  Learning resources                  Provider API production guides;
                                      OpenTelemetry; AWS Well-Architected

  Practical work                      Add per-request token/cost/latency
                                      logs; simulate provider timeout;
                                      implement fallback and request
                                      budget; create a dashboard or
                                      report

  Completion criteria                 Explain the cost and latency of a
                                      request; diagnose a failed call;
                                      demonstrate bounded retries,
                                      timeouts and useful telemetry

  Dependencies                        E2, E4, E5, E7
  -----------------------------------------------------------------------

### E10. FDE project: build one real AI-enabled application

  -----------------------------------------------------------------------
  Field                               Plan
  ----------------------------------- -----------------------------------
  Priority                            High

  Time estimate                       12--15 hours across the six months

  Topics                              End-to-end applied AI engineering

  Subtopics                           Pick one real workflow; identify
                                      user/problem; build a small API;
                                      integrate a model; add retrieval or
                                      tools only when needed; persist
                                      data; add
                                      authentication/authorization if
                                      applicable; evaluate behavior; add
                                      observability; document deployment
                                      and limitations

  Learning resources                  Use the resources in E1--E9;
                                      provider documentation should be
                                      the source of truth for API
                                      behavior

  Practical work                      Build a knowledge assistant,
                                      personal productivity assistant,
                                      support copilot, document Q&A tool,
                                      or another bounded workflow.
                                      Suggested implementation:
                                      Python/FastAPI AI service,
                                      Java/Spring Boot integration if
                                      useful, PostgreSQL/pgvector,
                                      Docker, and a simple UI

  Completion criteria                 A working deployed or locally
                                      reproducible product; README and
                                      architecture diagram;
                                      test/evaluation dataset;
                                      cost/latency report; security
                                      considerations; demo of failure
                                      handling

  Dependencies                        E1--E9 progressively
  -----------------------------------------------------------------------

### FDE project milestones

  -----------------------------------------------------------------------------
  Milestone               Target phase            Deliverable
  ----------------------- ----------------------- -----------------------------
  M1 --- Problem          Weeks 1--4              One-page problem statement,
  definition                                      user journey, success metric,
                                                  architecture sketch

  M2 --- Basic model      Weeks 5--8              Working API call, structured
  integration                                     output, validation, logs

  M3 --- Useful           Weeks 9--13             RAG or tool-calling vertical
  context/tools                                   slice; do not add both unless
                                                  the use case needs both

  M4 --- Reliability and  Weeks 14--18            Golden dataset, regression
  evaluation                                      evaluation, error handling,
                                                  authorization and telemetry

  M5 --- Production       Weeks 19--22            Cost/latency controls,
  hardening                                       deployment/reproducibility,
                                                  documentation

  M6 --- Technical        Weeks 23--26            Architecture explanation,
  walkthrough                                     demo, trade-offs, known
                                                  limitations and next steps
  -----------------------------------------------------------------------------

**Mentor warning:** Do not start with a multi-agent system. Start with a
direct model call, then structured output, then retrieval/tool use, then
a deterministic workflow. Introduce an agent framework only when dynamic
planning is genuinely required.

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

  SQL and transactions    Java, HLD and FDE       Persistence, isolation,
                                                  idempotency and vector
                                                  metadata

  Networking and APIs     HLD and FDE             Model APIs,
                                                  service-to-service
                                                  communication, timeouts
                                                  and retries

  Queues and event-driven HLD, Java and FDE       Async ingestion,
  design                                          background jobs and
                                                  tool execution

  Testing and             All tracks              Evidence that
  observability                                   code/design works and
                                                  can be operated

  SOLID and design        LLD and Java            Extensible
  patterns                                        integrations, provider
                                                  abstractions and clean
                                                  service boundaries

  Distributed systems     HLD and FDE             Failure handling,
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
            Week Java                 DSA --- official    LLD --- CodeArmy            HLD                      FDE
                                      NeetCode 250                                                             
  -------------- -------------------- ------------------- --------------------------- ------------------------ ----------------------------
               1 A1 language/object   Resume first        C1 approach and OOP         D1 requirements and      E1 Python setup and API
                 model baseline       incomplete official                             estimation               basics
                                      category; baseline                                                       
                                      1--2 problems                                                            

               2 A2 collections       Continue official   C1 UML and modeling         D2 request lifecycle     E1 FastAPI, typing, tests
                 foundations          order; arrays/hash                                                       
                                      and/or next                                                              
                                      incomplete topic                                                         

               3 A2 HashMap internals Continue official   C2 SOLID                    D3 SQL/data modeling     E2 model API integration
                 and comparators      order; two                                                               
                                      pointers/sliding                                                         
                                      window as                                                                
                                      applicable                                                               

               4 A3 streams and       Continue official   C2 refactoring exercise     D3 indexes/transactions  E2 streaming, timeouts, cost
                 modern Java          order; binary                                                            
                                      search/linked list                                                       
                                      as applicable                                                            

               5 A4 threads and race  Continue official   C3                          D4 caching               E3 structured outputs
                 conditions           order; linked       factory/builder/singleton                            
                                      list/stack as                                                            
                                      applicable                                                               

               6 A4 synchronization   Continue official   C3 implement patterns       D4 cache invalidation    E4 tool calling
                 and JMM              order; stack/tree                                                        
                                      as applicable                                                            

               7 A5 locks,            Continue official   C4 adapter/decorator/facade D5 queues and Kafka      E4 tool execution and
                 conditions,          order; trees/tries                              concepts                 idempotency
                 executors            as applicable                                                            

               8 A5 CompletableFuture Continue official   C4 proxy/composite          D5 retries, DLQ, outbox  E5 embeddings and chunking
                 and virtual threads  order;                                                                   
                                      backtracking/heap                                                        
                                      as applicable                                                            

               9 A6 JVM memory and    Continue official   C5 strategy/observer        D6 consistency and       E5 pgvector retrieval
                 class loading        order; graph                                    replication              
                                      foundations as                                                           
                                      applicable                                                               

              10 A6 GC and memory     Continue official   C5 command/state/chain      D6 partial failures      E5 RAG evaluation
                 troubleshooting      order; graphs                                                            

              11 A7 JFR and profiling Continue official   C6 first independent        D7 scaling and           E6 workflow state
                                      order; advanced     project                     availability             
                                      graphs                                                                   

              12 A8 Spring DI and     Continue official   C6 second independent       D7 rate                  E6 checkpointing/HITL
                 bean lifecycle       order; 1-D DP       project                     limiting/resilience      

              13 A8 proxies and       Continue official   C6 third project            D8                       E7 golden dataset
                 transactions         order; 1-D DP                                   observability/security   

              14 A9 SQL               Continue official   C6 fourth project           D8 SLOs and tracing      E7 regression evaluation
                 isolation/indexing   order; 2-D DP                                                            

              15 A9 idempotency and   Continue official   C7 testing and concurrency  D9 URL                   E8 prompt injection and
                 locking              order; 2-D DP                                   shortener/notification   isolation
                                                                                      design                   

              16 A10 tests and        Continue official   C7 timed machine coding     D9 feed/chat design      E8 tool authorization
                 production concerns  order;                                                                   
                                      greedy/intervals                                                         

              17 Java revision:       Continue official   Rebuild weakest LLD project D9 payment/order design  E9 cost and latency
                 collections +        order; remaining                                                         
                 concurrency          greedy/intervals                                                         

              18 Java revision: JVM + Continue official   Pattern selection drills    D9 search/scheduler      E9 tracing/fallbacks
                 Spring               order; bit                                      design                   
                                      manipulation                                                             

              19 Java mock interview  Close remaining     60--90 min machine coding   D10 architecture         E10 project hardening
                                      official DSA                                    trade-offs               
                                      categories as                                                            
                                      feasible                                                                 

              20 Weak-area            Timed NeetCode      LLD mock 1                  HLD mock 1               E10
                 remediation          re-solves                                                                deployment/reproducibility

              21 Java concurrency     Timed NeetCode      LLD mock 2                  HLD mock 2               E10 evaluation report
                 mock                 re-solves                                                                

              22 Spring/JVM deep      Weak-problem        LLD mock 3                  HLD mock 3               E10 security review
                 revision             revision                                                                 

              23 Java mixed interview Mixed timed set     Explain project design      Design unfamiliar system Project technical
                 set                  from NeetCode only                                                       walkthrough

              24 Java flash recall +  Re-solve weak       Rebuild one project from    Revise weak design areas Fix weakest evaluation cases
                 coding               high-priority       blank                                                
                                      problems                                                                 

              25 Full Java mock       Timed DSA mock      Timed LLD mock              Timed HLD mock           Cost/latency and failure
                                                                                                               drill

              26 Final interview      Final NeetCode 250  Final design review         Final design review      Final demo and architecture
                 consolidation        status + weak list                                                       review
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
  FDE concept + implementation             45--90 min
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

  FDE                     Working increments,     One coherent AI-enabled
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
5.  What did I ship in the FDE project?
6.  What is the single biggest blocker for next week?
7.  Is the current weekly workload sustainable?

### Monthly checkpoint

At the end of each four-week block: - Take one Java mock or oral
question session. - Take one timed DSA session using only NeetCode 250
problems. - Complete one LLD or HLD design exercise. - Demonstrate one
tangible FDE project improvement. - Remove or defer low-value topics if
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

## FDE / AI Engineering

-   [OpenAI developer
    documentation](https://developers.openai.com/api/docs/) --- APIs,
    Responses, tools, structured outputs and embeddings.
-   [Anthropic --- Building effective
    agents](https://www.anthropic.com/research/building-effective-agents)
    --- workflow/agent distinction and simplicity-first design.
-   [LangGraph
    documentation](https://docs.langchain.com/oss/python/langgraph/) ---
    stateful workflows, persistence and orchestration.
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
> maintainable services, and can build production-minded AI
> applications.**

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
documentation, Anthropic's agent engineering guidance, LangGraph
documentation and pgvector documentation.

-   NeetCode 250 official counts and topic groups:
    https://neetcode.io/practice/practice/neetcode250
-   Coder Army LLD course code and playlist reference:
    https://github.com/adityatandon15/Low-Level-Design-Course
-   System Design Primer:
    https://github.com/donnemartin/system-design-primer
-   Anthropic agent guidance:
    https://www.anthropic.com/research/building-effective-agents
-   Java learning resources: https://dev.java/learn/
-   AWS Well-Architected:
    https://docs.aws.amazon.com/wellarchitected/latest/userguide/waf.html
-   OpenAI API documentation: https://developers.openai.com/api/docs/
-   pgvector: https://github.com/pgvector/pgvector
