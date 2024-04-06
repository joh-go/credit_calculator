use thousands::Separable;

pub struct AnnuityLoan {
    loan: f64,
    annual_pay_off_rate: f64,
    annual_interest_rate: f64,
    monthly_interest_rate: f64,
    monthly_annuity: f64,
    begin_pay_off: chrono::NaiveDate,
}

pub struct Rate {
    pub payment_date: chrono::NaiveDate,
    pub annuity: f64,
    pub interest: f64,
    pub amortization: f64,
    pub unscheduled_amortization: f64,
    pub residual_dept: f64,
}

impl Rate {
    pub fn to_string(&self) -> String {
        format!(
            "Date: {} Annuity: {:.2}€ Interest: {:.2}€ Amortization: {:.2}€ Residual Dept: {:.2}€",
            self.payment_date, self.annuity, self.interest, self.amortization, self.residual_dept
        )
    }
}

impl AnnuityLoan {
    pub fn from_loan_rate(
        loan: f64,
        annual_interest_rate_percentage: f64,
        annual_pay_off_rate_percentage: f64,
    ) -> AnnuityLoan {
        let annual_interest_rate = annual_interest_rate_percentage / 100f64;
        let annual_pay_off_rate = annual_pay_off_rate_percentage / 100f64;

        AnnuityLoan {
            loan: loan,
            annual_pay_off_rate: annual_pay_off_rate,
            annual_interest_rate: annual_interest_rate,
            monthly_interest_rate: AnnuityLoan::calculate_monthly_interest_rate(
                annual_interest_rate,
            ),
            monthly_annuity: AnnuityLoan::calculate_monthly_annuity(
                loan,
                annual_interest_rate,
                annual_pay_off_rate,
            ),
            begin_pay_off: chrono::NaiveDate::from_ymd_opt(2024, 1, 1).unwrap(),
        }
    }

    pub fn calculate_pay_off_plan(&self) -> Vec<Rate> {
        let mut interest = self.loan * self.monthly_interest_rate;
        let mut amortization = self.monthly_annuity - interest;

        let mut rates: Vec<Rate> = vec![Rate {
            payment_date: self.begin_pay_off,
            annuity: self.monthly_annuity,
            interest: interest,
            amortization: amortization,
            unscheduled_amortization: 0f64,
            residual_dept: self.loan - amortization,
        }];

        let mut last_rate = rates.last().unwrap();
        while last_rate.residual_dept > 0f64 {
            interest = last_rate.residual_dept * self.monthly_interest_rate;
            amortization = self.monthly_annuity - interest;
            amortization = match amortization < last_rate.residual_dept {
                true => amortization,
                false => last_rate.residual_dept,
            };

            let monthly_annuity = match amortization < last_rate.residual_dept {
                true => self.monthly_annuity,
                false => amortization + interest,
            };

            rates.push(Rate {
                payment_date: last_rate.payment_date + chrono::Months::new(1),
                annuity: monthly_annuity,
                interest: interest,
                amortization: amortization,
                unscheduled_amortization: 0f64,
                residual_dept: last_rate.residual_dept - amortization,
            });

            last_rate = rates.last().unwrap();
        }

        rates
    }

    pub fn to_string(&self) -> String {
        let rates = self.calculate_pay_off_plan();

        let years_to_pay_off = rates.len() / 12;

        let month_to_pay_off = rates.len() % 12;

        format!(
            "Loan: {}\nInterestRate: {:.2}%\nFirstPayOffRate: {:.2}%\nMonthly annuity: {:.2}€\nPayed off in {} year(s) and {} month(s)",
            self.loan.round().separate_with_spaces(),
            self.annual_interest_rate * 100f64,
            self.annual_pay_off_rate * 100f64,
            self.monthly_annuity,
            years_to_pay_off,
            month_to_pay_off
        )
    }

    fn calculate_monthly_annuity(
        loan: f64,
        annual_interest_rate: f64,
        annual_pay_off_rate: f64,
    ) -> f64 {
        loan / 12f64 * (annual_pay_off_rate + annual_interest_rate)
    }

    fn calculate_monthly_interest_rate(annual_interest_rate: f64) -> f64 {
        annual_interest_rate / 12f64
    }
}
