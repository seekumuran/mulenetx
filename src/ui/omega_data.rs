use std::sync::OnceLock;

pub const COMPANIES: [&str; 10] = [
    "AAPL", "MSFT", "NVDA", "AMZN", "GOOGL", "META", "TSLA", "AVGO", "ORCL", "AMD",
];

pub const ACCOUNT_COUNT: usize = 10_000;
pub const TRANSACTION_COUNT: usize = 100_000;

#[derive(Clone, Debug)]
pub struct Account {
    pub id: usize,
    pub company: &'static str,
    pub cluster: usize,
    pub risk: u8,
    pub exposure: u64,
    pub signal: &'static str,
}

#[derive(Clone, Debug)]
pub struct Transaction {
    pub id: usize,
    pub source: usize,
    pub target: usize,
    pub amount: u64,
    pub minute: u32,
    pub signal: &'static str,
}

pub struct Network {
    pub accounts: Vec<Account>,
    pub transactions: Vec<Transaction>,
}

static NETWORK: OnceLock<Network> = OnceLock::new();

fn mix(mut x: u64) -> u64 {
    x ^= x >> 30;
    x = x.wrapping_mul(0xbf58476d1ce4e5b9);
    x ^= x >> 27;
    x = x.wrapping_mul(0x94d049bb133111eb);
    x ^ (x >> 31)
}

pub fn network() -> &'static Network {
    NETWORK.get_or_init(generate)
}

fn generate() -> Network {
    let mut accounts = Vec::with_capacity(ACCOUNT_COUNT);
    let mut exposure = vec![0_u64; ACCOUNT_COUNT];
    let mut degree = vec![0_u32; ACCOUNT_COUNT];
    let mut velocity = vec![0_u32; ACCOUNT_COUNT];
    let mut circular = vec![0_u32; ACCOUNT_COUNT];

    for id in 0..ACCOUNT_COUNT {
        accounts.push(Account {
            id,
            company: COMPANIES[id % COMPANIES.len()],
            cluster: id / 100,
            risk: 0,
            exposure: 0,
            signal: "BASELINE",
        });
    }

    let mut transactions = Vec::with_capacity(TRANSACTION_COUNT);

    for id in 0..TRANSACTION_COUNT {
        let h = mix(id as u64 + 0x0A6E_6A01);
        let source = (h as usize) % ACCOUNT_COUNT;

        // Most transfers stay inside a cluster; some cross clusters.
        let target = if id % 5 == 0 {
            let other_cluster = (source / 100 + 1 + (h as usize % 99)) % (ACCOUNT_COUNT / 100);
            other_cluster * 100 + (mix(h) as usize % 100)
        } else {
            (source / 100) * 100 + ((source % 100 + 1 + h as usize % 99) % 100)
        };

        let target = if target == source {
            (target + 1) % ACCOUNT_COUNT
        } else {
            target
        };

        let amount = 5_000 + mix(h ^ 0x51A7) % 2_000_000;
        let minute = (id as u32) % (30 * 24 * 60);

        exposure[source] = exposure[source].saturating_add(amount);
        exposure[target] = exposure[target].saturating_add(amount / 2);
        degree[source] += 1;
        degree[target] += 1;

        if id % 97 == 0 {
            velocity[source] += 1;
        }
        if id % 211 == 0 {
            circular[source] += 1;
        }

        let signal = if id % 211 == 0 {
            "CIRCULAR"
        } else if id % 97 == 0 {
            "VELOCITY"
        } else if source / 100 != target / 100 {
            "CROSS-CLUSTER"
        } else {
            "TRANSFER"
        };

        transactions.push(Transaction {
            id,
            source,
            target,
            amount,
            minute,
            signal,
        });
    }

    for account in &mut accounts {
        let id = account.id;
        let score = 15
            + (degree[id] / 2).min(25)
            + (velocity[id] * 10).min(30)
            + (circular[id] * 15).min(30);

        account.risk = score.min(100) as u8;
        account.exposure = exposure[id];

        account.signal = if circular[id] > 0 {
            "CIRCULAR"
        } else if velocity[id] > 0 {
            "VELOCITY"
        } else if degree[id] > 25 {
            "HIGH-DEGREE"
        } else if degree[id] > 15 {
            "DENSE-LINK"
        } else {
            "BASELINE"
        };
    }

    Network {
        accounts,
        transactions,
    }
}
