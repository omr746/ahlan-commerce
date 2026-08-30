# DAL Alternatives: ORM vs SQL-First Code Generation

### Goal



The main question for this task is:

> Should we use an ORM such as Diesel or SeaORM instead of writing SQL directly?

ORMs are valid and useful in many projects, but they are not the default choice for Ahlan Commerce.

Ahlan Commerce uses **SQL-first code generation** because we want SQL to remain the primary source of truth while using code generation to reduce boilerplate and manual mapping.

---

## Comparison

| Option                        | Pros                                                                                                    | Cons                                                                                                 | What it protects against                                                                               | What it does not protect against                                                                     |
| ----------------------------- | ------------------------------------------------------------------------------------------------------- | ---------------------------------------------------------------------------------------------------- | ------------------------------------------------------------------------------------------------------ | ---------------------------------------------------------------------------------------------------- |
| **Diesel**                    | Strong ORM, type-safe, compile-time checks, good performance, and safe query building                   | Learning curve, higher abstraction than SQL, complex SQL can be less straightforward                 | Many errors involving columns, types, and query construction                                           | Does not prevent bad business logic, poor database design, or make every complex query easy          |
| **SeaORM**                    | Modern Rust ORM, async support, useful for CRUD, handles relationships, reduces boilerplate             | Higher abstraction, generated entities can add complexity, actual SQL can be less visible            | Many model/type errors and incorrect CRUD operations                                                   | Does not prevent business-rule errors or poor database design                                        |
| **SQLx checked queries**      | SQL remains explicit, compile-time query checking, async support, good PostgreSQL integration           | More manual mapping/code, more boilerplate, database is needed for query checking                    | SQL syntax errors, invalid columns, and many SQL/Rust type mismatches                                  | Does not generate a complete DAL automatically or prevent business-logic mistakes                    |
| **SQL-first code generation** | SQL is the source of truth, schema is explicit, generated code reduces boilerplate, SQL remains visible | Requires discipline and a generation workflow                                                        | Reduces mapping mistakes and repetitive DAL code; catches query/schema/type problems during generation | Does not prevent bad SQL design, business-logic errors, transaction mistakes, or inefficient queries |
| **Cornucopia**                | SQL-first, generates Rust types/functions from SQL, reduces manual mapping, keeps SQL explicit          | Requires an organized workflow, generation must be part of development, less abstraction than an ORM | SQL/result mapping issues and many type mismatches; makes query contracts explicit                     | Does not prevent poor query design, business-logic errors, transaction problems, or inefficient SQL  |

---

# Why Is an ORM a Valid Choice?

An ORM is **not a bad design choice**.

In many applications, an ORM can be very useful when the project has:

* A lot of CRUD operations.
* Many database relationships.
* A need to reduce manually written SQL.
* A desire for a higher-level abstraction over the database.
* Many simple database operations.
* A preference for working mainly with models and entities.

For example, for a simple entity:

```text
Product
 ├── id
 ├── name
 ├── price
 └── description
```

An ORM can provide operations such as:

```text
create
find
update
delete
```

without requiring us to manually write SQL for every operation.

Therefore:

> **ORM is valid. It is not an anti-pattern.**

However, being valid does not mean that it is the best default for every project.

---

# Why Is ORM Not the Default in Ahlan Commerce?

Ahlan Commerce follows a **database-centric approach**.

We want SQL to be explicit and intentional rather than having the ORM abstraction generate most of the SQL for us.

The project follows this model:

```text
SQL
 ↓
Cornucopia
 ↓
Generated Rust code
 ↓
Application
```

Instead of:

```text
Application
 ↓
ORM abstraction
 ↓
Generated SQL
 ↓
Database
```

The main difference is:

> With SQL-first, the developer starts from SQL and explicitly decides what the database should execute.

This fits Ahlan Commerce because we want:

1. Explicit control over SQL.
2. Clear visibility into what PostgreSQL executes.
3. Database schema and queries to remain visible parts of the design.
4. Less unnecessary abstraction.
5. Code generation to reduce repetitive DAL code.
6. A type-safe Rust API on top of explicitly written SQL.

---

# Why Cornucopia?

Cornucopia fits this project because it combines two important ideas.

## 1. SQL-first

We write SQL ourselves:

```sql
SELECT
    id,
    handle,
    name,
    description,
    price,
    published_at
FROM products
WHERE id = $1;
```

The SQL remains visible, explicit, and easy to review.

## 2. Code Generation

Instead of manually writing Rust structs and result mapping for every query, Cornucopia generates the required Rust code.

Conceptually:

```text
SQL Query
   ↓
Cornucopia
   ↓
Generated Rust Types + Functions
```

This reduces boilerplate and keeps the DAL consistent.

---

# SQLx Checked Queries vs Cornucopia

SQLx and Cornucopia share an important principle:

> SQL is not hidden behind an ORM abstraction.

With SQLx, we can write something like:

```rust
sqlx::query_as!(
    Product,
    r#"
        SELECT id, name, price
        FROM products
        WHERE id = $1
    "#,
    id
)
```

The SQL remains explicit and is checked against the database.

However, the developer still writes and maintains much of the query and mapping code.

With Cornucopia, SQL becomes the input to a code-generation process:

```text
SQL files
   ↓
Cornucopia
   ↓
Generated DAL
```

This makes Cornucopia a particularly good fit for Ahlan Commerce's goal of exploring a **SQL-first code-generation approach**.

---

# What Do We Get from SQL-First + Cornucopia?

The goal is not simply to write SQL manually.

The goal is to combine the advantages of explicit SQL and code generation:

```text
        SQL-first
           +
    Code Generation
           ↓
      Type-safe DAL
```

This gives us:

* Explicit SQL.
* Better visibility into the database layer.
* Generated types.
* Less repetitive code.
* Less manual result mapping.
* Clear contracts between SQL and Rust.
* Earlier detection of certain query and type problems.

---

# What Does SQL-First NOT Protect Us From?

It is important not to treat code generation as a solution to every database problem.

Cornucopia can help detect issues such as:

```text
SQL ↔ Rust type mismatch
SQL ↔ database schema mismatch
Missing/incorrect selected columns
Invalid SQL
```

But it cannot know whether the query actually implements the correct business requirement.

For example:

```sql
SELECT *
FROM products
WHERE price > 0;
```

This SQL can be completely valid.

Cornucopia may successfully generate code for it.

But suppose the business requirement is:

> Only published products should be returned.

Then the query is wrong from a business perspective.

Therefore:

```text
Valid SQL
   ≠
Correct business logic
```

---

# Why Does SQL-First Code Generation Still Require Discipline?

Type safety and code generation do not mean that developers can stop thinking about the database layer.

We still need to follow good practices.

## 1. Write Clear SQL

We should not write unnecessarily complex or confusing SQL simply because it compiles.

## 2. Review Generated Code

Generated code reduces manual work, but developers still need to understand and review the original SQL.

## 3. Pay Attention to Indexes

A query can be completely correct from a type and syntax perspective while still being very slow because the required indexes are missing.

```text
Correct query
     ≠
Efficient query
```

## 4. Handle Transactions Correctly

Code generation does not prevent incorrect transaction boundaries or transaction-related business logic.

## 5. Test Business Behavior

Even if a query compiles successfully, we still need tests to verify that it returns the correct results.

## 6. Keep Migrations and Queries in Sync

When the database schema changes, the related SQL queries and generated code may need to change as well.

The workflow should be:

```text
Migration
   ↓
Database schema changes
   ↓
Review/update SQL queries
   ↓
Regenerate code
   ↓
Run tests
```

---

# What Does Each Layer Protect Us From?

It is useful to think about database safety as multiple layers:

```text
Database
   ↓
SQL correctness
   ↓
Type checking / code generation
   ↓
Rust compiler
   ↓
Tests
   ↓
Business correctness
```

Each layer solves a different category of problems.

Cornucopia does not solve everything.

The Rust compiler does not understand business requirements.

Tests do not automatically prevent poor database design.

Therefore, these tools should be used together.

---

# Why Ahlan Commerce Uses SQL-First Generated Code with Cornucopia

The main reason is that this approach matches the technical and learning goals of the project.

Ahlan Commerce is designed to help us understand:

* How to work directly with PostgreSQL.
* How to write clear SQL.
* How to design database queries.
* How to connect SQL results with Rust types.
* How to reduce manual mapping using code generation.
* How to maintain type safety without hiding SQL behind an ORM abstraction.

The architecture is therefore:

```text
PostgreSQL
    ↓
SQL
    ↓
Cornucopia
    ↓
Generated Rust DAL
    ↓
Application
```

Rather than making an ORM the primary abstraction for database access.

---

# Final Conclusion

ORMs such as **Diesel** and **SeaORM** are valid choices and can be excellent solutions for many projects.

**SQLx checked queries** are also a strong option when we want to write SQL manually while getting compile-time verification.

However, Ahlan Commerce chose **SQL-first code generation with Cornucopia** because we want to combine:

```text
Explicit SQL
     +
Type Safety
     +
Generated Code
     +
Low Boilerplate
```

The key idea is:

> **SQL-first does not mean writing everything manually.**

It means that SQL is the explicit source of truth, while code generation produces the Rust code needed to connect those queries to the application.

At the same time:

> **Code generation does not make the database layer automatically safe.**

We still need discipline around:

* SQL design
* Database schema
* Migrations
* Indexes
* Transactions
* Business logic
* Testing
* Performance

Therefore, an ORM is a valid option, but it is not the default path for Ahlan Commerce.

**SQL-first + Cornucopia** is the better fit because it provides explicit SQL control while reducing boilerplate and maintaining a type-safe generated Rust DAL.
