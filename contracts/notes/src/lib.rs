#![no_std]

use soroban_sdk::{
    contract, contractimpl, contracttype, symbol_short, Address, Env, Map, String, Vec,
};

const DEFAULT_WARNING_BPS: u32 = 8_000;
const BPS_DENOMINATOR: i128 = 10_000;
const MAX_SIMULATION_YEARS: u32 = 100;

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum EntryKind {
    Income,
    Expense,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum NeedWantTag {
    Need,
    Want,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum BudgetAlert {
    Safe,
    Warning,
    Exceeded,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub enum DataKey {
    TxSeq(Address),
    Tx(Address, u64),
    GoalSeq(Address),
    Goal(Address, u64),
    BudgetList(Address),
    Assets(Address),
    Debts(Address),
    FreedomConfig(Address),
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Transaction {
    pub id: u64,
    pub date: u64,                 // format: YYYYMMDD
    pub amount: i128,
    pub category: String,
    pub payment_method: String,
    pub note: String,
    pub kind: EntryKind,
    pub tag: NeedWantTag,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetPlan {
    pub category: String,
    pub monthly_limit: i128,
    pub spent_amount: i128,
    pub warning_threshold_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BudgetStatus {
    pub category: String,
    pub monthly_limit: i128,
    pub spent_amount: i128,
    pub remaining_amount: i128,
    pub usage_bps: u32,
    pub alert: BudgetAlert,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FinancialGoal {
    pub id: u64,
    pub name: String,
    pub target_amount: i128,
    pub saved_amount: i128,
    pub deadline: u64,             // format: YYYYMMDD
    pub priority: u32,
    pub recurring_deposit: i128,
    pub active: bool,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AssetPortfolio {
    pub cash: i128,
    pub savings: i128,
    pub ewallet: i128,
    pub crypto: i128,
    pub stocks: i128,
    pub gold: i128,
    pub other: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DebtPortfolio {
    pub personal_loan: i128,
    pub credit_card: i128,
    pub paylater: i128,
    pub installments: i128,
    pub other: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct NetWorthSummary {
    pub total_assets: i128,
    pub total_debts: i128,
    pub net_worth: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct DashboardSummary {
    pub total_assets: i128,
    pub liquid_balance: i128,
    pub monthly_income: i128,
    pub monthly_expense: i128,
    pub saving_rate_bps: u32,
    pub debt_ratio_bps: u32,
    pub progress_target_bps: u32,
    pub financial_health_score: u32,
    pub net_worth: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct CashFlowAnalysis {
    pub largest_expense: i128,
    pub largest_expense_category: String,
    pub most_wasteful_category: String,
    pub cheapest_month: u32,       // format: YYYYMM
    pub recurring_expense_estimate: i128,
    pub potential_savings: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreedomConfig {
    pub monthly_living_cost: i128,
    pub annual_passive_yield_bps: u32,
    pub conservative_growth_bps: u32,
    pub moderate_growth_bps: u32,
    pub aggressive_growth_bps: u32,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreedomScenario {
    pub scenario: String,
    pub monthly_saving: i128,
    pub annual_growth_bps: u32,
    pub years_estimate: u32,
    pub target_assets: i128,
    pub target_passive_income: i128,
}

#[contracttype]
#[derive(Clone, Debug, Eq, PartialEq)]
pub struct FreedomProjection {
    pub monthly_living_cost: i128,
    pub target_passive_income: i128,
    pub target_productive_assets: i128,
    pub current_productive_assets: i128,
    pub estimated_years: u32,
    pub conservative: FreedomScenario,
    pub moderate: FreedomScenario,
    pub aggressive: FreedomScenario,
    pub message: String,
}

#[contract]
pub struct FinancialFreedomContract;

#[contractimpl]
impl FinancialFreedomContract {
    // =========================
    // 1. Asset & Debt Tracker
    // =========================
    pub fn set_assets(env: Env, user: Address, assets: AssetPortfolio) {
        user.require_auth();
        ensure_non_negative_assets(&assets);
        env.storage()
            .persistent()
            .set(&DataKey::Assets(user.clone()), &assets);
        env.events()
            .publish((symbol_short!("assets"), user), assets);
    }

    pub fn get_assets(env: Env, user: Address) -> AssetPortfolio {
        read_assets(&env, &user)
    }

    pub fn set_debts(env: Env, user: Address, debts: DebtPortfolio) {
        user.require_auth();
        ensure_non_negative_debts(&debts);
        env.storage()
            .persistent()
            .set(&DataKey::Debts(user.clone()), &debts);
        env.events()
            .publish((symbol_short!("debts"), user), debts);
    }

    pub fn get_debts(env: Env, user: Address) -> DebtPortfolio {
        read_debts(&env, &user)
    }

    pub fn get_net_worth(env: Env, user: Address) -> NetWorthSummary {
        let assets = read_assets(&env, &user);
        let debts = read_debts(&env, &user);
        let total_assets = calc_total_assets(&assets);
        let total_debts = calc_total_debts(&debts);

        NetWorthSummary {
            total_assets,
            total_debts,
            net_worth: total_assets - total_debts,
        }
    }

    // =========================
    // 2. Income & Expense Log
    // =========================
    pub fn add_transaction(
        env: Env,
        user: Address,
        date: u64,
        amount: i128,
        category: String,
        payment_method: String,
        note: String,
        kind: EntryKind,
        tag: NeedWantTag,
    ) -> u64 {
        user.require_auth();

        if amount <= 0 {
            panic!("amount must be > 0");
        }
        validate_date(date);

        let seq_key = DataKey::TxSeq(user.clone());
        let last_id: u64 = env.storage().persistent().get(&seq_key).unwrap_or(0);
        let next_id = last_id + 1;

        let tx = Transaction {
            id: next_id,
            date,
            amount,
            category: category.clone(),
            payment_method,
            note,
            kind: kind.clone(),
            tag,
        };

        env.storage().persistent().set(&seq_key, &next_id);
        env.storage()
            .persistent()
            .set(&DataKey::Tx(user.clone(), next_id), &tx);

        if matches_expense(&kind) {
            apply_budget_spent(&env, &user, &category, amount);
        }

        env.events()
            .publish((symbol_short!("tx_add"), user), tx);

        next_id
    }

    pub fn get_transaction(env: Env, user: Address, tx_id: u64) -> Transaction {
        let key = DataKey::Tx(user, tx_id);
        env.storage().persistent().get(&key).unwrap()
    }

    pub fn list_transactions(env: Env, user: Address) -> Vec<Transaction> {
        let max_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::TxSeq(user.clone()))
            .unwrap_or(0);

        let mut out = Vec::new(&env);
        let mut i: u64 = 1;
        while i <= max_id {
            let key = DataKey::Tx(user.clone(), i);
            if env.storage().persistent().has(&key) {
                let tx: Transaction = env.storage().persistent().get(&key).unwrap();
                out.push_back(tx);
            }
            i += 1;
        }
        out
    }

    // =========================
    // 3. Monthly Budgeting
    // =========================
    pub fn upsert_budget_default(env: Env, user: Address, category: String, monthly_limit: i128) {
        Self::upsert_budget(env, user, category, monthly_limit, DEFAULT_WARNING_BPS);
    }

    pub fn upsert_budget(
        env: Env,
        user: Address,
        category: String,
        monthly_limit: i128,
        warning_threshold_bps: u32,
    ) {
        user.require_auth();

        if monthly_limit <= 0 {
            panic!("monthly_limit must be > 0");
        }
        if warning_threshold_bps == 0 || warning_threshold_bps > 10_000 {
            panic!("warning_threshold_bps must be between 1 and 10000");
        }

        let mut budgets = read_budget_list(&env, &user);
        let mut found = false;
        let len = budgets.len();
        let mut i: u32 = 0;

        while i < len {
            let mut item = budgets.get(i).unwrap();
            if item.category == category {
                item.monthly_limit = monthly_limit;
                item.warning_threshold_bps = warning_threshold_bps;
                budgets.set(i, item);
                found = true;
                break;
            }
            i += 1;
        }

        if !found {
            budgets.push_back(BudgetPlan {
                category,
                monthly_limit,
                spent_amount: 0,
                warning_threshold_bps,
            });
        }

        write_budget_list(&env, &user, &budgets);
    }

    pub fn list_budgets(env: Env, user: Address) -> Vec<BudgetStatus> {
        let budgets = read_budget_list(&env, &user);
        let mut out = Vec::new(&env);
        let len = budgets.len();
        let mut i: u32 = 0;

        while i < len {
            let item = budgets.get(i).unwrap();
            out.push_back(budget_to_status(item));
            i += 1;
        }

        out
    }

    pub fn get_budget(env: Env, user: Address, category: String) -> BudgetStatus {
        let budgets = read_budget_list(&env, &user);
        let len = budgets.len();
        let mut i: u32 = 0;

        while i < len {
            let item = budgets.get(i).unwrap();
            if item.category == category {
                return budget_to_status(item);
            }
            i += 1;
        }

        panic!("budget category not found");
    }

    pub fn reset_budget_spent(env: Env, user: Address) {
        user.require_auth();

        let mut budgets = read_budget_list(&env, &user);
        let len = budgets.len();
        let mut i: u32 = 0;

        while i < len {
            let mut item = budgets.get(i).unwrap();
            item.spent_amount = 0;
            budgets.set(i, item);
            i += 1;
        }

        write_budget_list(&env, &user, &budgets);
    }

    // =========================
    // 4. Financial Goals
    // =========================
    pub fn create_goal(
        env: Env,
        user: Address,
        name: String,
        target_amount: i128,
        deadline: u64,
        priority: u32,
        recurring_deposit: i128,
    ) -> u64 {
        user.require_auth();

        if target_amount <= 0 {
            panic!("target_amount must be > 0");
        }
        if recurring_deposit < 0 {
            panic!("recurring_deposit must be >= 0");
        }
        validate_date(deadline);

        let seq_key = DataKey::GoalSeq(user.clone());
        let last_id: u64 = env.storage().persistent().get(&seq_key).unwrap_or(0);
        let next_id = last_id + 1;

        let goal = FinancialGoal {
            id: next_id,
            name,
            target_amount,
            saved_amount: 0,
            deadline,
            priority,
            recurring_deposit,
            active: true,
        };

        env.storage().persistent().set(&seq_key, &next_id);
        env.storage()
            .persistent()
            .set(&DataKey::Goal(user.clone(), next_id), &goal);

        env.events()
            .publish((symbol_short!("goal_new"), user), goal);

        next_id
    }

    pub fn deposit_to_goal(env: Env, user: Address, goal_id: u64, amount: i128) {
        user.require_auth();

        if amount <= 0 {
            panic!("amount must be > 0");
        }

        let key = DataKey::Goal(user.clone(), goal_id);
        let mut goal: FinancialGoal = env.storage().persistent().get(&key).unwrap();

        if !goal.active {
            panic!("goal is inactive");
        }

        goal.saved_amount += amount;
        if goal.saved_amount > goal.target_amount {
            goal.saved_amount = goal.target_amount;
        }

        env.storage().persistent().set(&key, &goal);
        env.events()
            .publish((symbol_short!("goal_top"), user), goal);
    }

    pub fn close_goal(env: Env, user: Address, goal_id: u64) {
        user.require_auth();

        let key = DataKey::Goal(user.clone(), goal_id);
        let mut goal: FinancialGoal = env.storage().persistent().get(&key).unwrap();
        goal.active = false;
        env.storage().persistent().set(&key, &goal);
    }

    pub fn get_goal(env: Env, user: Address, goal_id: u64) -> FinancialGoal {
        env.storage()
            .persistent()
            .get(&DataKey::Goal(user, goal_id))
            .unwrap()
    }

    pub fn list_goals(env: Env, user: Address) -> Vec<FinancialGoal> {
        let max_id: u64 = env
            .storage()
            .persistent()
            .get(&DataKey::GoalSeq(user.clone()))
            .unwrap_or(0);

        let mut out = Vec::new(&env);
        let mut i: u64 = 1;
        while i <= max_id {
            let key = DataKey::Goal(user.clone(), i);
            if env.storage().persistent().has(&key) {
                let goal: FinancialGoal = env.storage().persistent().get(&key).unwrap();
                out.push_back(goal);
            }
            i += 1;
        }
        out
    }

    // =========================
    // 5. Dashboard
    // =========================
    pub fn get_dashboard(env: Env, user: Address, year: u32, month: u32) -> DashboardSummary {
        if month == 0 || month > 12 {
            panic!("month must be between 1 and 12");
        }

        let assets = read_assets(&env, &user);
        let debts = read_debts(&env, &user);
        let (monthly_income, monthly_expense) = monthly_cash_flow(&env, &user, year, month);

        let total_assets = calc_total_assets(&assets);
        let total_debts = calc_total_debts(&debts);
        let liquid_balance = calc_liquid_assets(&assets);
        let net_worth = total_assets - total_debts;
        let saving_rate_bps = calc_saving_rate_bps(monthly_income, monthly_expense);
        let debt_ratio_bps = ratio_bps(total_debts, total_assets);
        let progress_target_bps = calc_goal_progress_bps(&env, &user);
        let financial_health_score = calc_financial_health_score(
            saving_rate_bps,
            debt_ratio_bps,
            liquid_balance,
            monthly_expense,
            progress_target_bps,
        );

        DashboardSummary {
            total_assets,
            liquid_balance,
            monthly_income,
            monthly_expense,
            saving_rate_bps,
            debt_ratio_bps,
            progress_target_bps,
            financial_health_score,
            net_worth,
        }
    }

    // =========================
    // 6. Cash Flow Analysis
    // =========================
    pub fn analyze_cash_flow(env: Env, user: Address) -> CashFlowAnalysis {
        let txs = Self::list_transactions(env.clone(), user);
        let mut largest_expense: i128 = 0;
        let mut largest_expense_category = empty_string(&env);
        let mut most_wasteful_category = empty_string(&env);
        let mut cheapest_month: u32 = 0;
        let mut recurring_expense_estimate: i128 = 0;
        let mut potential_savings: i128 = 0;

        let mut category_total: Map<String, i128> = Map::new(&env);
        let mut category_count: Map<String, u32> = Map::new(&env);
        let mut month_total: Map<u32, i128> = Map::new(&env);

        let len = txs.len();
        let mut i: u32 = 0;
        while i < len {
            let tx = txs.get(i).unwrap();
            if matches_expense(&tx.kind) {
                if tx.amount > largest_expense {
                    largest_expense = tx.amount;
                    largest_expense_category = tx.category.clone();
                }

                let current_total = category_total.get(tx.category.clone()).unwrap_or(0);
                category_total.set(tx.category.clone(), current_total + tx.amount);

                let current_count = category_count.get(tx.category.clone()).unwrap_or(0);
                category_count.set(tx.category.clone(), current_count + 1);

                let ym = year_month_from_date(tx.date);
                let month_sum = month_total.get(ym).unwrap_or(0);
                month_total.set(ym, month_sum + tx.amount);

                if matches_want(&tx.tag) {
                    potential_savings += tx.amount / 2;
                }
            }
            i += 1;
        }

        let mut category_iter = category_total.keys();
        while let Some(category) = category_iter.next() {
            let total = category_total.get(category.clone()).unwrap_or(0);
            let count = category_count.get(category.clone()).unwrap_or(0);
            if total > category_total.get(most_wasteful_category.clone()).unwrap_or(0) {
                most_wasteful_category = category.clone();
            }
            if count >= 2 {
                recurring_expense_estimate += total / (count as i128);
            }
        }

        let mut month_iter = month_total.keys();
        let mut smallest_seen: i128 = 0;
        let mut has_month = false;
        while let Some(ym) = month_iter.next() {
            let total = month_total.get(ym).unwrap_or(0);
            if !has_month || total < smallest_seen {
                smallest_seen = total;
                cheapest_month = ym;
                has_month = true;
            }
        }

        CashFlowAnalysis {
            largest_expense,
            largest_expense_category,
            most_wasteful_category,
            cheapest_month,
            recurring_expense_estimate,
            potential_savings,
        }
    }

    // =========================
    // 7. Financial Freedom
    // =========================
    pub fn set_freedom_config(env: Env, user: Address, config: FreedomConfig) {
        user.require_auth();

        if config.monthly_living_cost <= 0 {
            panic!("monthly_living_cost must be > 0");
        }
        if config.annual_passive_yield_bps == 0 {
            panic!("annual_passive_yield_bps must be > 0");
        }

        env.storage()
            .persistent()
            .set(&DataKey::FreedomConfig(user.clone()), &config);

        env.events()
            .publish((symbol_short!("ff_cfg"), user), config);
    }

    pub fn get_freedom_config(env: Env, user: Address) -> FreedomConfig {
        env.storage()
            .persistent()
            .get(&DataKey::FreedomConfig(user))
            .unwrap()
    }

    pub fn calculate_financial_freedom(env: Env, user: Address) -> FreedomProjection {
        let config: FreedomConfig = env
            .storage()
            .persistent()
            .get(&DataKey::FreedomConfig(user.clone()))
            .unwrap();

        let assets = read_assets(&env, &user);
        let current_productive_assets = calc_productive_assets(&assets);
        let (latest_year, latest_month) = latest_transaction_month(&env, &user);
        let (income, expense) = if latest_year == 0 || latest_month == 0 {
            (0, 0)
        } else {
            monthly_cash_flow(&env, &user, latest_year, latest_month)
        };

        let base_monthly_saving = positive_part(income - expense);
        let target_passive_income = config.monthly_living_cost;
        let annual_target_income = target_passive_income * 12;
        let target_productive_assets = (annual_target_income * BPS_DENOMINATOR)
            / (config.annual_passive_yield_bps as i128);

        let conservative = build_scenario(
            &env,
            "Conservative",
            base_monthly_saving * 90 / 100,
            config.conservative_growth_bps,
            current_productive_assets,
            target_productive_assets,
            target_passive_income,
        );

        let moderate = build_scenario(
            &env,
            "Moderate",
            base_monthly_saving,
            config.moderate_growth_bps,
            current_productive_assets,
            target_productive_assets,
            target_passive_income,
        );

        let aggressive = build_scenario(
            &env,
            "Aggressive",
            base_monthly_saving * 110 / 100,
            config.aggressive_growth_bps,
            current_productive_assets,
            target_productive_assets,
            target_passive_income,
        );

        FreedomProjection {
            monthly_living_cost: config.monthly_living_cost,
            target_passive_income,
            target_productive_assets,
            current_productive_assets,
            estimated_years: moderate.years_estimate,
            conservative,
            moderate,
            aggressive,
            message: String::from_str(
                &env,
                "Dengan saving rate dan growth saat ini, lihat estimated_years untuk estimasi target financial freedom.",
            ),
        }
    }
}

// =========================
// Helper functions
// =========================

fn empty_string(env: &Env) -> String {
    String::from_str(env, "")
}

fn zero_assets() -> AssetPortfolio {
    AssetPortfolio {
        cash: 0,
        savings: 0,
        ewallet: 0,
        crypto: 0,
        stocks: 0,
        gold: 0,
        other: 0,
    }
}

fn zero_debts() -> DebtPortfolio {
    DebtPortfolio {
        personal_loan: 0,
        credit_card: 0,
        paylater: 0,
        installments: 0,
        other: 0,
    }
}

fn read_assets(env: &Env, user: &Address) -> AssetPortfolio {
    env.storage()
        .persistent()
        .get(&DataKey::Assets(user.clone()))
        .unwrap_or(zero_assets())
}

fn read_debts(env: &Env, user: &Address) -> DebtPortfolio {
    env.storage()
        .persistent()
        .get(&DataKey::Debts(user.clone()))
        .unwrap_or(zero_debts())
}

fn read_budget_list(env: &Env, user: &Address) -> Vec<BudgetPlan> {
    env.storage()
        .persistent()
        .get(&DataKey::BudgetList(user.clone()))
        .unwrap_or(Vec::new(env))
}

fn write_budget_list(env: &Env, user: &Address, budgets: &Vec<BudgetPlan>) {
    env.storage()
        .persistent()
        .set(&DataKey::BudgetList(user.clone()), budgets);
}

fn calc_total_assets(assets: &AssetPortfolio) -> i128 {
    assets.cash
        + assets.savings
        + assets.ewallet
        + assets.crypto
        + assets.stocks
        + assets.gold
        + assets.other
}

fn calc_liquid_assets(assets: &AssetPortfolio) -> i128 {
    assets.cash + assets.savings + assets.ewallet
}

fn calc_productive_assets(assets: &AssetPortfolio) -> i128 {
    assets.savings + assets.crypto + assets.stocks + assets.gold + assets.other
}

fn calc_total_debts(debts: &DebtPortfolio) -> i128 {
    debts.personal_loan + debts.credit_card + debts.paylater + debts.installments + debts.other
}

fn ratio_bps(numerator: i128, denominator: i128) -> u32 {
    if numerator <= 0 || denominator <= 0 {
        return 0;
    }
    ((numerator * BPS_DENOMINATOR) / denominator) as u32
}

fn calc_saving_rate_bps(income: i128, expense: i128) -> u32 {
    if income <= 0 {
        return 0;
    }
    let net = positive_part(income - expense);
    ((net * BPS_DENOMINATOR) / income) as u32
}

fn calc_goal_progress_bps(env: &Env, user: &Address) -> u32 {
    let goals = FinancialFreedomContract::list_goals(env.clone(), user.clone());
    let len = goals.len();
    let mut i: u32 = 0;
    let mut total_target: i128 = 0;
    let mut total_saved: i128 = 0;

    while i < len {
        let goal = goals.get(i).unwrap();
        if goal.active {
            total_target += goal.target_amount;
            total_saved += goal.saved_amount;
        }
        i += 1;
    }

    ratio_bps(total_saved, total_target)
}

fn calc_financial_health_score(
    saving_rate_bps: u32,
    debt_ratio_bps: u32,
    liquid_assets: i128,
    monthly_expense: i128,
    goal_progress_bps: u32,
) -> u32 {
    let mut score: u32 = 0;

    // Saving rate: max 35 points
    let saving_points = if saving_rate_bps >= 2_000 {
        35
    } else {
        (saving_rate_bps * 35) / 2_000
    };
    score += saving_points;

    // Debt ratio: max 30 points
    let debt_points = if debt_ratio_bps <= 3_000 {
        30
    } else if debt_ratio_bps >= 10_000 {
        0
    } else {
        ((10_000 - debt_ratio_bps) * 30) / 7_000
    };
    score += debt_points;

    // Liquidity / emergency fund: max 20 points
    let months_cover = if monthly_expense > 0 {
        liquid_assets / monthly_expense
    } else {
        0
    };
    let liquidity_points = if months_cover >= 6 {
        20
    } else if months_cover <= 0 {
        0
    } else {
        (months_cover as u32 * 20) / 6
    };
    score += liquidity_points;

    // Goal progress: max 15 points
    let goal_points = if goal_progress_bps >= 5_000 {
        15
    } else {
        (goal_progress_bps * 15) / 5_000
    };
    score += goal_points;

    score
}

fn monthly_cash_flow(env: &Env, user: &Address, year: u32, month: u32) -> (i128, i128) {
    let txs = FinancialFreedomContract::list_transactions(env.clone(), user.clone());
    let len = txs.len();
    let mut income: i128 = 0;
    let mut expense: i128 = 0;
    let mut i: u32 = 0;

    while i < len {
        let tx = txs.get(i).unwrap();
        if year_from_date(tx.date) == year && month_from_date(tx.date) == month {
            if matches_income(&tx.kind) {
                income += tx.amount;
            } else {
                expense += tx.amount;
            }
        }
        i += 1;
    }

    (income, expense)
}

fn latest_transaction_month(env: &Env, user: &Address) -> (u32, u32) {
    let txs = FinancialFreedomContract::list_transactions(env.clone(), user.clone());
    let len = txs.len();
    let mut latest_ym: u32 = 0;
    let mut i: u32 = 0;

    while i < len {
        let tx = txs.get(i).unwrap();
        let ym = year_month_from_date(tx.date);
        if ym > latest_ym {
            latest_ym = ym;
        }
        i += 1;
    }

    if latest_ym == 0 {
        (0, 0)
    } else {
        (latest_ym / 100, latest_ym % 100)
    }
}

fn apply_budget_spent(env: &Env, user: &Address, category: &String, amount: i128) {
    let mut budgets = read_budget_list(env, user);
    let len = budgets.len();
    let mut i: u32 = 0;

    while i < len {
        let mut item = budgets.get(i).unwrap();
        if item.category == category.clone() {
            item.spent_amount += amount;
            budgets.set(i, item);
            write_budget_list(env, user, &budgets);
            return;
        }
        i += 1;
    }
}

fn budget_to_status(item: BudgetPlan) -> BudgetStatus {
    let remaining_amount = item.monthly_limit - item.spent_amount;
    let usage_bps = ratio_bps(item.spent_amount, item.monthly_limit);

    let alert = if item.spent_amount > item.monthly_limit {
        BudgetAlert::Exceeded
    } else if usage_bps >= item.warning_threshold_bps {
        BudgetAlert::Warning
    } else {
        BudgetAlert::Safe
    };

    BudgetStatus {
        category: item.category,
        monthly_limit: item.monthly_limit,
        spent_amount: item.spent_amount,
        remaining_amount,
        usage_bps,
        alert,
    }
}

fn build_scenario(
    env: &Env,
    scenario_name: &str,
    monthly_saving: i128,
    annual_growth_bps: u32,
    current_assets: i128,
    target_assets: i128,
    target_passive_income: i128,
) -> FreedomScenario {
    let years_estimate = simulate_years(
        positive_part(monthly_saving),
        current_assets,
        target_assets,
        annual_growth_bps,
    );

    FreedomScenario {
        scenario: String::from_str(env, scenario_name),
        monthly_saving: positive_part(monthly_saving),
        annual_growth_bps,
        years_estimate,
        target_assets,
        target_passive_income,
    }
}

fn simulate_years(
    monthly_saving: i128,
    current_assets: i128,
    target_assets: i128,
    annual_growth_bps: u32,
) -> u32 {
    if current_assets >= target_assets {
        return 0;
    }

    let mut assets = current_assets;
    let annual_contribution = monthly_saving * 12;
    let mut years: u32 = 0;

    while years < MAX_SIMULATION_YEARS {
        assets += annual_contribution;
        assets += (assets * annual_growth_bps as i128) / BPS_DENOMINATOR;
        years += 1;

        if assets >= target_assets {
            return years;
        }
    }

    MAX_SIMULATION_YEARS
}

fn positive_part(value: i128) -> i128 {
    if value > 0 { value } else { 0 }
}

fn validate_date(date: u64) {
    let year = year_from_date(date);
    let month = month_from_date(date);
    let day = (date % 100) as u32;

    if year < 2000 || month == 0 || month > 12 || day == 0 || day > 31 {
        panic!("invalid date format, use YYYYMMDD");
    }
}

fn year_from_date(date: u64) -> u32 {
    (date / 10_000) as u32
}

fn month_from_date(date: u64) -> u32 {
    ((date / 100) % 100) as u32
}

fn year_month_from_date(date: u64) -> u32 {
    (year_from_date(date) * 100) + month_from_date(date)
}

fn matches_income(kind: &EntryKind) -> bool {
    matches!(kind, EntryKind::Income)
}

fn matches_expense(kind: &EntryKind) -> bool {
    matches!(kind, EntryKind::Expense)
}

fn matches_want(tag: &NeedWantTag) -> bool {
    matches!(tag, NeedWantTag::Want)
}

fn ensure_non_negative_assets(assets: &AssetPortfolio) {
    if assets.cash < 0
        || assets.savings < 0
        || assets.ewallet < 0
        || assets.crypto < 0
        || assets.stocks < 0
        || assets.gold < 0
        || assets.other < 0
    {
        panic!("asset values must be >= 0");
    }
}

fn ensure_non_negative_debts(debts: &DebtPortfolio) {
    if debts.personal_loan < 0
        || debts.credit_card < 0
        || debts.paylater < 0
        || debts.installments < 0
        || debts.other < 0
    {
        panic!("debt values must be >= 0");
    }
}
