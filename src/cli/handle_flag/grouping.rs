static CATEGORY_KEYWORDS: [&str; 3] = [
    "c", "cat", "category"
];

static GROUP_KEYWORDS: [&str; 3]    = [
    "g", "grp", "group"
];

static PERIOD_KEYWORDS: [&str; 4]   = [
    "p", "pd", "per", "period"
];

static BLOCK_KEYWORDS: [&str; 4]    = [
    "b", "bk", "blk", "block"
];


pub fn parse(number:u8, args: String) {
    if CATEGORY_KEYWORDS.contains(&&args[..]) {
        category(number);
    }

    if GROUP_KEYWORDS.contains(&&args[..]) {
        group(number);
    }

    if PERIOD_KEYWORDS.contains(&&args[..]) {
        period(number);
    }

    if BLOCK_KEYWORDS.contains(&&args[..]) {
        block(number);
    }
}

fn category(number: u8) {
    todo!(
        "call output::grouping::category"
    )
}

fn group(number: u8) {
    todo!(
        "call output::grouping::group"
    )
}

fn period(number: u8) {
    todo!(
        "call output::grouping::period"
    )
}

fn block(number: u8) {
    todo!(
        "call output::grouping::block"
    )
}

// Special Funcs

fn help(number: u8) {
    todo!(
        "print help info"
    )
}

fn incomplete(number: u8) {
    todo!(
        "print wrong keyword and then, help"
    )
}

fn all(number: u8) {
    todo!(
        "call output::grouping::all"
    )
}
