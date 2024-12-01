use anyhow::Result as Anyhow;
use chrono::{Datelike, Local, NaiveDate};
use inquire::{validator::Validation, CustomType, DateSelect, Select};

use crate::config::{Account, Config, Customer, Supplier};

// Prompt
#[derive(Debug)]
pub(crate) struct Prompt {
    pub(crate) invoice_number: u32,
    pub(crate) issue_date: NaiveDate,
    pub(crate) service_date: NaiveDate,
    pub(crate) due_date: NaiveDate,
    pub(crate) supplier: Supplier,
    pub(crate) service: String,
    pub(crate) amount: f32,
    pub(crate) account: Account,
    pub(crate) customer: Customer,
}

impl Prompt {
    pub(crate) fn ask(config: &Config) -> Anyhow<Self> {
        let today = Local::now().date_naive();
        Ok(Self {
            invoice_number: Self::ask_invoice_number()?,
            issue_date: Self::ask_issue_date(&today)?,
            service_date: Self::ask_service_date(&today)?,
            due_date: Self::ask_due_date(&today)?,
            supplier: Self::ask_supplier(config)?,
            service: Self::ask_service(config)?,
            amount: Self::ask_amount()?,
            account: Self::ask_account(config)?,
            customer: Self::ask_customer(config)?,
        })
    }

    fn ask_invoice_number() -> Anyhow<u32> {
        Ok(CustomType::<u32>::new("What's the invoice number?")
            .with_default(1)
            .with_validator(|val: &u32| {
                if *val < 1 {
                    Ok(Validation::Invalid("The number must be >= 1".into()))
                } else {
                    Ok(Validation::Valid)
                }
            })
            .with_error_message("Please type a valid number")
            .with_help_message(
                "Type the number of the invoice (the number starts from 1 each month). It's used \
                for invoice ID.",
            )
            .prompt()?)
    }

    fn ask_issue_date(today: &NaiveDate) -> Anyhow<NaiveDate> {
        Ok(DateSelect::new("Please, provide the invoice issue date.")
            .with_default(*today)
            .with_week_start(chrono::Weekday::Mon)
            .prompt()?)
    }

    fn ask_service_date(today: &NaiveDate) -> Anyhow<NaiveDate> {
        let last_day = NaiveDate::from_ymd_opt(today.year(), (today.month() + 1).min(12), 1)
            .expect("Error getting last day")
            - chrono::Days::new(1);
        Ok(DateSelect::new("Please, provide the service date.")
            .with_default(last_day)
            .with_week_start(chrono::Weekday::Mon)
            .with_help_message("The service date is usually the last day of the reporting month.")
            .prompt()?)
    }

    fn ask_due_date(today: &NaiveDate) -> Anyhow<NaiveDate> {
        let due_date = NaiveDate::from_ymd_opt(today.year(), (today.month() + 1).min(12), 5)
            .expect("Error getting next month");
        Ok(DateSelect::new("Please, provide the due date.")
            .with_default(due_date)
            .with_week_start(chrono::Weekday::Mon)
            .prompt()?)
    }

    fn ask_supplier(config: &Config) -> Anyhow<Supplier> {
        Ok(Select::new(
            "Select the supplier, which provided the service.",
            config.suppliers.clone(),
        )
        .with_help_message("You can add suppliers in the config file.")
        .prompt()?)
    }

    fn ask_service(config: &Config) -> Anyhow<String> {
        Ok(Select::new(
            "Select the the service you provided.",
            config.services.clone(),
        )
        .with_help_message("You can add services in the config file.")
        .prompt()?)
    }

    fn ask_amount() -> Anyhow<f32> {
        Ok(CustomType::<f32>::new("What's the amount?")
            .with_formatter(&|i| format!("{:.2}", i))
            .with_error_message("Please type a valid number")
            .with_help_message("Type the amount using a decimal point as a separator")
            .prompt()?)
    }

    fn ask_account(config: &Config) -> Anyhow<Account> {
        Ok(
            Select::new("Please, select your bank account.", config.accounts.clone())
                .with_help_message("You can add accounts in the config file.")
                .prompt()?,
        )
    }

    fn ask_customer(config: &Config) -> Anyhow<Customer> {
        Ok(
            Select::new("Please, select the customer.", config.customers.clone())
                .with_help_message("You can add customers in the config file.")
                .prompt()?,
        )
    }
}
