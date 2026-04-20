# Management Financial to Financial Freedom

Smart contract Web3 berbasis **Soroban** untuk membantu pengguna mengelola keuangan pribadi dan menghitung jalur menuju **financial freedom**.

This is a **Soroban-based Web3 smart contract** designed to help users manage personal finances and estimate their path toward **financial freedom**.

---

# English

## 1. Overview

This project is a Soroban smart contract that stores and computes personal finance data on-chain. The contract focuses on core financial logic, including:

- asset and debt tracking
- income and expense recording
- category-based budgeting
- financial goal management
- financial dashboard summary
- simple cash flow analysis
- financial freedom calculation

The contract follows a simple model: **1 wallet address = 1 user profile**.

---

## 2. Project Objective

This smart contract is designed to help users:

- understand their current financial position
- monitor monthly income and expenses
- control spending through category budgets
- track financial goals in a structured way
- calculate net worth from total assets and total debts
- estimate how long it may take to reach financial freedom

---

## 3. Implemented Features

### A. Personal Finance Dashboard
The contract provides a dashboard summary through `get_dashboard()`.

Displayed metrics:
- total assets
- liquid balance
- monthly income
- monthly expense
- saving rate
- debt ratio
- financial goal progress
- financial health score
- net worth

Implementation notes:
- `liquid balance` is calculated from `cash + savings + ewallet`
- `saving rate` is based on monthly surplus relative to income
- `debt ratio` is total debt relative to total assets
- `goal progress` is active saved amount relative to active target amount
- `financial health score` is derived from saving rate, debt ratio, liquidity, and goal progress

### B. Income and Expense Recording
The contract provides `add_transaction()` to record financial transactions.

Transaction fields:
- `date` in `YYYYMMDD` format
- `amount`
- `category`
- `payment_method`
- `note`
- `kind` → `Income` or `Expense`
- `tag` → `Need` or `Want`

Related methods:
- `add_transaction()`
- `get_transaction()`
- `list_transactions()`

Implementation notes:
- `amount` must be greater than 0
- if the transaction is an `Expense`, the contract automatically adds the amount to the matching budget category if that budget already exists

### C. Monthly Budgeting
The contract supports category-based budgeting.

Related methods:
- `upsert_budget_default()`
- `upsert_budget()`
- `list_budgets()`
- `get_budget()`
- `reset_budget_spent()`

Stored budget data:
- category
- monthly limit
- current spent amount
- warning threshold

Calculated budget status:
- realized spending
- remaining budget
- usage percentage
- alert status: `Safe`, `Warning`, or `Exceeded`

Important implementation notes:
- budget usage is cumulative until `reset_budget_spent()` is called
- the contract does not automatically reset budgets each month
- the default warning threshold is `8000 bps` or `80%`

### D. Financial Goals
The contract supports financial goal creation.

Example use cases:
- emergency fund
- vacation
- house down payment
- laptop purchase
- business capital
- financial freedom target

Related methods:
- `create_goal()`
- `deposit_to_goal()`
- `close_goal()`
- `get_goal()`
- `list_goals()`

Goal fields:
- `name`
- `target_amount`
- `saved_amount`
- `deadline`
- `priority`
- `recurring_deposit`
- `active`

Implementation notes:
- `saved_amount` is capped at `target_amount`
- a goal can be closed with `close_goal()`
- dashboard goal progress only counts active goals

### E. Cash Flow Analysis
The contract provides simple cash flow analysis through `analyze_cash_flow()`.

Analysis outputs:
- largest expense
- category of the largest expense
- most wasteful category
- cheapest month
- recurring expense estimate
- potential savings

Logic used:
- `largest_expense` is the highest single expense transaction
- `most_wasteful_category` is the category with the highest total expense
- `cheapest_month` is the month with the lowest total expense
- `recurring_expense_estimate` is derived from expense categories that appear at least twice
- `potential_savings` is calculated as 50% of all transactions tagged as `Want`

Note:
- this is a lightweight on-chain analytics version
- deeper historical analytics should be handled by an indexer or an off-chain backend

### F. Net Worth Tracker
The contract supports tracking assets, debts, and net worth.

Related methods:
- `set_assets()`
- `get_assets()`
- `set_debts()`
- `get_debts()`
- `get_net_worth()`

Asset components:
- cash
- savings
- ewallet
- crypto
- stocks
- gold
- other

Debt components:
- personal_loan
- credit_card
- paylater
- installments
- other

Implementation notes:
- all asset and debt values must be `>= 0`
- `net_worth = total_assets - total_debts`

### G. Financial Freedom Calculator
This is the core feature of the project and is available through:
- `set_freedom_config()`
- `get_freedom_config()`
- `calculate_financial_freedom()`

Stored configuration:
- `monthly_living_cost`
- `annual_passive_yield_bps`
- `conservative_growth_bps`
- `moderate_growth_bps`
- `aggressive_growth_bps`

Main outputs:
- monthly living cost
- target monthly passive income
- target productive assets
- current productive assets
- estimated years to financial freedom
- conservative scenario
- moderate scenario
- aggressive scenario
- summary message

Core logic:
- target passive income = monthly living cost
- target productive assets are derived from annual passive income needs divided by annual yield
- current productive assets are calculated from `savings + crypto + stocks + gold + other`
- base monthly saving is taken from the latest transaction month
- estimated years are simulated using annual contributions and asset growth until the target is reached

Implementation notes:
- if no transactions exist, base monthly saving is treated as 0
- the default headline estimate uses the moderate scenario
- the simulation is capped at 100 years

---

## 4. Main Data Structures

### Enums
- `EntryKind` → `Income`, `Expense`
- `NeedWantTag` → `Need`, `Want`
- `BudgetAlert` → `Safe`, `Warning`, `Exceeded`

### Structs
- `Transaction`
- `BudgetPlan`
- `BudgetStatus`
- `FinancialGoal`
- `AssetPortfolio`
- `DebtPortfolio`
- `NetWorthSummary`
- `DashboardSummary`
- `CashFlowAnalysis`
- `FreedomConfig`
- `FreedomScenario`
- `FreedomProjection`

### Storage Keys
The contract uses `DataKey` for persistent storage:
- `TxSeq(Address)`
- `Tx(Address, u64)`
- `GoalSeq(Address)`
- `Goal(Address, u64)`
- `BudgetList(Address)`
- `Assets(Address)`
- `Debts(Address)`
- `FreedomConfig(Address)`

---

## 5. Public Methods

### Asset and Debt
- `set_assets(env, user, assets)`
- `get_assets(env, user)`
- `set_debts(env, user, debts)`
- `get_debts(env, user)`
- `get_net_worth(env, user)`

### Transactions
- `add_transaction(env, user, date, amount, category, payment_method, note, kind, tag)`
- `get_transaction(env, user, tx_id)`
- `list_transactions(env, user)`

### Budget
- `upsert_budget_default(env, user, category, monthly_limit)`
- `upsert_budget(env, user, category, monthly_limit, warning_threshold_bps)`
- `list_budgets(env, user)`
- `get_budget(env, user, category)`
- `reset_budget_spent(env, user)`

### Goals
- `create_goal(env, user, name, target_amount, deadline, priority, recurring_deposit)`
- `deposit_to_goal(env, user, goal_id, amount)`
- `close_goal(env, user, goal_id)`
- `get_goal(env, user, goal_id)`
- `list_goals(env, user)`

### Dashboard and Analysis
- `get_dashboard(env, user, year, month)`
- `analyze_cash_flow(env, user)`

### Financial Freedom
- `set_freedom_config(env, user, config)`
- `get_freedom_config(env, user)`
- `calculate_financial_freedom(env, user)`

---

## 6. Important Validation Rules

The contract enforces the following validations:

- transaction amount must be `> 0`
- budget `monthly_limit` must be `> 0`
- `warning_threshold_bps` must be between `1` and `10000`
- goal `target_amount` must be `> 0`
- goal `recurring_deposit` must be `>= 0`
- `monthly_living_cost` must be `> 0`
- `annual_passive_yield_bps` must be `> 0`
- all asset and debt values must be `>= 0`
- date format must be `YYYYMMDD`
- year must be at least `2000`
- month must be `1-12`
- day must be `1-31`

---

## 7. Authorization

All methods that modify user data use `user.require_auth()`.

This means:
- only the owner of the address can change their own data
- read methods such as `get_assets()` or `list_transactions()` do not require authorization, but still require a user address as input

---

## 8. Published Events

The contract publishes events for the following important actions:

- `assets`
- `debts`
- `tx_add`
- `goal_new`
- `goal_top`
- `ff_cfg`

These events can be consumed by an indexer or backend service to synchronize data and build off-chain dashboards.

---

## 9. Current Limitations

Current version limitations include:

1. **Monthly budget is not reset automatically**  
   Budget reset must be triggered manually using `reset_budget_spent()`.

2. **Cash flow analytics are still simple**  
   The analysis is based on on-chain transaction iteration and does not yet support deeper historical insight.

3. **No transaction or goal deletion workflow**  
   The contract supports creation, reading, and goal closing, but not transaction editing or deletion.

4. **No month-specific budget storage model yet**  
   The contract stores a running `spent_amount` per category.

5. **Financial freedom uses the latest transaction month**  
   The estimate strongly depends on how complete the user’s transaction records are.

---

## 10. Suggested Future Improvements

To make the project stronger for production, the next iterations may include:

- automatic monthly budget reset
- monthly budget history
- transaction edit and delete support
- default category registry and category validation
- more event coverage for important state changes
- frontend dashboard integration
- indexer or off-chain analytics integration
- inflation-aware financial freedom simulation
- more detailed financial health scoring
- recurring goal deposit reminders

---

## 11. Suggested Usage Flow

Recommended sequence:

1. user sets assets with `set_assets()`
2. user sets debts with `set_debts()`
3. user creates category budgets with `upsert_budget()`
4. user records income and expense transactions with `add_transaction()`
5. user creates financial goals with `create_goal()`
6. user sets financial freedom config with `set_freedom_config()`
7. frontend reads summaries through:
   - `get_dashboard()`
   - `list_budgets()`
   - `list_goals()`
   - `analyze_cash_flow()`
   - `get_net_worth()`
   - `calculate_financial_freedom()`

---

## 12. Summary

This smart contract is an on-chain foundation for a Soroban-based personal finance management application. Its primary focus is storing core financial data and computing important indicators that support the larger goal of **financial freedom**.

The current version is already strong enough as an **MVP smart contract**, and it can be extended further with a frontend, indexer, and off-chain analytics layer.

---

# Bahasa Indonesia

## 1. Gambaran Umum

Proyek ini adalah smart contract Soroban yang menyimpan dan menghitung data keuangan pribadi secara on-chain. Kontrak ini berfokus pada logika inti, yaitu:

- pencatatan aset dan utang
- pencatatan pemasukan dan pengeluaran
- pengelolaan anggaran per kategori
- pengelolaan target keuangan
- ringkasan dashboard keuangan
- analisis arus kas sederhana
- kalkulator financial freedom

Kontrak ini memakai pendekatan **1 wallet address = 1 profil pengguna**.

---

## 2. Tujuan Proyek

Smart contract ini dibuat untuk membantu pengguna:

- mengetahui posisi keuangan mereka saat ini
- memantau pemasukan dan pengeluaran bulanan
- mengontrol pengeluaran berdasarkan anggaran
- melacak target keuangan secara terstruktur
- melihat kekayaan bersih dari total aset dan total utang
- menghitung estimasi waktu menuju financial freedom

---

## 3. Fitur yang Sudah Diimplementasikan

### A. Dashboard Keuangan Pribadi
Kontrak menyediakan ringkasan dashboard melalui method `get_dashboard()`.

Data yang ditampilkan:
- total aset
- saldo likuid
- pemasukan bulanan
- pengeluaran bulanan
- saving rate
- debt ratio
- progress target keuangan
- skor kesehatan finansial
- net worth

Catatan implementasi:
- `saldo likuid` dihitung dari `cash + savings + ewallet`
- `saving rate` dihitung dari surplus bulanan dibanding pemasukan
- `debt ratio` dihitung dari total utang dibanding total aset
- `progress target` dihitung dari total tabungan target aktif dibanding total target aktif
- `financial health score` dihitung dari saving rate, debt ratio, likuiditas, dan progress goal

### B. Pencatatan Pemasukan dan Pengeluaran
Kontrak menyediakan method `add_transaction()` untuk mencatat transaksi.

Field transaksi:
- `date` dalam format `YYYYMMDD`
- `amount`
- `category`
- `payment_method`
- `note`
- `kind` → `Income` atau `Expense`
- `tag` → `Need` atau `Want`

Method terkait:
- `add_transaction()`
- `get_transaction()`
- `list_transactions()`

Catatan implementasi:
- nilai `amount` harus lebih dari 0
- jika transaksi adalah `Expense`, kontrak otomatis menambah `spent_amount` pada budget kategori yang sesuai bila budget kategori tersebut sudah ada

### C. Budgeting / Anggaran Bulanan
Kontrak menyediakan pengelolaan anggaran per kategori.

Method terkait:
- `upsert_budget_default()`
- `upsert_budget()`
- `list_budgets()`
- `get_budget()`
- `reset_budget_spent()`

Data budget yang disimpan:
- kategori
- batas bulanan
- total realisasi
- ambang warning

Status budget yang dihitung:
- realisasi
- sisa anggaran
- persentase penggunaan
- status alert: `Safe`, `Warning`, atau `Exceeded`

Catatan implementasi penting:
- budget bersifat akumulatif sampai `reset_budget_spent()` dipanggil
- kontrak belum melakukan reset otomatis per bulan
- default warning threshold adalah `8000 bps` atau `80%`

### D. Financial Goals
Kontrak mendukung pembuatan target keuangan.

Contoh target yang bisa dipakai:
- dana darurat
- liburan
- DP rumah
- laptop
- modal usaha
- financial freedom target

Method terkait:
- `create_goal()`
- `deposit_to_goal()`
- `close_goal()`
- `get_goal()`
- `list_goals()`

Field goal:
- `name`
- `target_amount`
- `saved_amount`
- `deadline`
- `priority`
- `recurring_deposit`
- `active`

Catatan implementasi:
- `saved_amount` tidak akan melebihi `target_amount`
- goal bisa ditutup dengan `close_goal()`
- progress dashboard hanya menghitung goal yang masih aktif

### E. Cash Flow Analysis
Kontrak menyediakan analisis arus kas sederhana melalui `analyze_cash_flow()`.

Output analisis:
- pengeluaran terbesar
- kategori pengeluaran terbesar
- kategori paling boros
- bulan paling hemat
- estimasi recurring expense
- potensi penghematan

Logika yang dipakai:
- `largest_expense` diambil dari transaksi expense terbesar
- `most_wasteful_category` diambil dari total expense kategori terbesar
- `cheapest_month` diambil dari bulan dengan total expense paling kecil
- `recurring_expense_estimate` dihitung dari kategori expense yang muncul minimal 2 kali
- `potential_savings` dihitung sebagai 50% dari total transaksi bertag `Want`

Catatan:
- analisis ini adalah versi awal yang ringan untuk on-chain
- untuk analitik historis yang lebih dalam, sebaiknya dibantu indexer atau backend off-chain

### F. Net Worth Tracker
Kontrak menyediakan pelacakan aset, utang, dan kekayaan bersih.

Method terkait:
- `set_assets()`
- `get_assets()`
- `set_debts()`
- `get_debts()`
- `get_net_worth()`

Komponen aset:
- cash
- savings
- ewallet
- crypto
- stocks
- gold
- other

Komponen utang:
- personal_loan
- credit_card
- paylater
- installments
- other

Catatan implementasi:
- semua nilai aset dan utang harus `>= 0`
- `net_worth = total_assets - total_debts`

### G. Financial Freedom Calculator
Fitur ini menjadi inti utama proyek dan diakses melalui:
- `set_freedom_config()`
- `get_freedom_config()`
- `calculate_financial_freedom()`

Konfigurasi yang disimpan:
- `monthly_living_cost`
- `annual_passive_yield_bps`
- `conservative_growth_bps`
- `moderate_growth_bps`
- `aggressive_growth_bps`

Output utama:
- biaya hidup bulanan
- target passive income bulanan
- target aset produktif
- aset produktif saat ini
- estimasi tahun menuju financial freedom
- skenario konservatif
- skenario moderat
- skenario agresif
- pesan ringkasan

Logika utama:
- target passive income = biaya hidup bulanan
- target aset produktif dihitung dari kebutuhan passive income tahunan dibagi yield tahunan
- aset produktif saat ini dihitung dari `savings + crypto + stocks + gold + other`
- saving bulanan dasar diambil dari bulan transaksi terbaru
- estimasi tahun dihitung dengan simulasi kontribusi tahunan dan pertumbuhan aset hingga target tercapai

Catatan implementasi:
- jika belum ada transaksi, saving bulanan dasar dianggap 0
- estimasi default utama memakai skenario moderat
- kontrak membatasi simulasi maksimal hingga 100 tahun

---

## 4. Struktur Data Utama

### Enum
- `EntryKind` → `Income`, `Expense`
- `NeedWantTag` → `Need`, `Want`
- `BudgetAlert` → `Safe`, `Warning`, `Exceeded`

### Struct
- `Transaction`
- `BudgetPlan`
- `BudgetStatus`
- `FinancialGoal`
- `AssetPortfolio`
- `DebtPortfolio`
- `NetWorthSummary`
- `DashboardSummary`
- `CashFlowAnalysis`
- `FreedomConfig`
- `FreedomScenario`
- `FreedomProjection`

### Storage Key
Kontrak memakai `DataKey` untuk penyimpanan persistent:
- `TxSeq(Address)`
- `Tx(Address, u64)`
- `GoalSeq(Address)`
- `Goal(Address, u64)`
- `BudgetList(Address)`
- `Assets(Address)`
- `Debts(Address)`
- `FreedomConfig(Address)`

---

## 5. Daftar Method Publik

### Asset dan Debt
- `set_assets(env, user, assets)`
- `get_assets(env, user)`
- `set_debts(env, user, debts)`
- `get_debts(env, user)`
- `get_net_worth(env, user)`

### Transaction
- `add_transaction(env, user, date, amount, category, payment_method, note, kind, tag)`
- `get_transaction(env, user, tx_id)`
- `list_transactions(env, user)`

### Budget
- `upsert_budget_default(env, user, category, monthly_limit)`
- `upsert_budget(env, user, category, monthly_limit, warning_threshold_bps)`
- `list_budgets(env, user)`
- `get_budget(env, user, category)`
- `reset_budget_spent(env, user)`

### Goals
- `create_goal(env, user, name, target_amount, deadline, priority, recurring_deposit)`
- `deposit_to_goal(env, user, goal_id, amount)`
- `close_goal(env, user, goal_id)`
- `get_goal(env, user, goal_id)`
- `list_goals(env, user)`

### Dashboard dan Analysis
- `get_dashboard(env, user, year, month)`
- `analyze_cash_flow(env, user)`

### Financial Freedom
- `set_freedom_config(env, user, config)`
- `get_freedom_config(env, user)`
- `calculate_financial_freedom(env, user)`

---

## 6. Aturan Validasi Penting

Kontrak menerapkan validasi berikut:

- nilai transaksi harus `> 0`
- `monthly_limit` budget harus `> 0`
- `warning_threshold_bps` harus antara `1` sampai `10000`
- `target_amount` goal harus `> 0`
- `recurring_deposit` goal harus `>= 0`
- `monthly_living_cost` harus `> 0`
- `annual_passive_yield_bps` harus `> 0`
- semua aset dan utang harus `>= 0`
- format tanggal harus `YYYYMMDD`
- tahun minimal `2000`
- bulan harus `1-12`
- hari harus `1-31`

---

## 7. Otorisasi

Semua method yang mengubah data user memakai `user.require_auth()`.

Artinya:
- hanya pemilik address yang dapat mengubah datanya sendiri
- method baca seperti `get_assets()` atau `list_transactions()` tidak memaksa otorisasi, tetapi tetap membutuhkan address user sebagai input

---

## 8. Event yang Dipublish

Kontrak mempublish event pada aksi penting berikut:

- `assets`
- `debts`
- `tx_add`
- `goal_new`
- `goal_top`
- `ff_cfg`

Event ini dapat dipakai oleh indexer atau backend untuk sinkronisasi data dan pembuatan dashboard off-chain.

---

## 9. Batasan Implementasi Saat Ini

Beberapa batasan versi saat ini:

1. **Budget bulanan belum reset otomatis**  
   Reset harus dipanggil manual melalui `reset_budget_spent()`.

2. **Analitik cash flow masih sederhana**  
   Analisis masih berbasis iterasi transaksi on-chain dan belum mendukung insight historis yang kompleks.

3. **Belum ada penghapusan transaksi atau goal**  
   Saat ini kontrak mendukung penambahan, pembacaan, dan penutupan goal, tetapi belum mendukung delete/update detail transaksi.

4. **Belum ada pemisahan budget per bulan di storage**  
   Budget saat ini menyimpan `spent_amount` berjalan.

5. **Financial freedom memakai bulan transaksi terbaru**  
   Jadi estimasi sangat bergantung pada kelengkapan data transaksi yang dimasukkan user.

---

## 10. Rekomendasi Pengembangan Lanjutan

Agar proyek ini lebih kuat untuk production, pengembangan selanjutnya bisa mencakup:

- reset budget otomatis berbasis bulan
- histori budget per bulan
- edit dan hapus transaksi
- kategori bawaan dan validasi kategori standar
- event tambahan untuk semua perubahan penting
- integrasi frontend dashboard
- integrasi indexer/off-chain analytics
- simulasi financial freedom berbasis inflasi dan kenaikan biaya hidup
- skor kesehatan finansial yang lebih detail
- reminder setoran target keuangan

---

## 11. Alur Penggunaan Singkat

Urutan penggunaan yang disarankan:

1. user mengisi aset melalui `set_assets()`
2. user mengisi utang melalui `set_debts()`
3. user membuat budget kategori melalui `upsert_budget()`
4. user mencatat transaksi income dan expense melalui `add_transaction()`
5. user membuat target keuangan melalui `create_goal()`
6. user mengisi konfigurasi financial freedom melalui `set_freedom_config()`
7. frontend membaca ringkasan melalui:
   - `get_dashboard()`
   - `list_budgets()`
   - `list_goals()`
   - `analyze_cash_flow()`
   - `get_net_worth()`
   - `calculate_financial_freedom()`

---

## 12. Ringkasan

Smart contract ini adalah fondasi on-chain untuk aplikasi manajemen keuangan pribadi berbasis Soroban. Fokus utamanya adalah menyimpan data inti dan menghitung indikator penting yang mendukung tujuan **financial freedom**.

Versi saat ini sudah cukup kuat sebagai **MVP smart contract**, dan dapat dikembangkan lebih lanjut dengan frontend, indexer, serta analytics off-chain.

---