pub mod credit_calculator;

slint::include_modules!();

use chrono::Datelike;
use slint::{StandardListViewItem, VecModel};
use std::rc::Rc;
use thousands::Separable;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_credit_plan({
        let ui_handle = ui.as_weak();
        move |loan, annual_interest_rate, annual_pay_off_rate, annual_unscheduled_amortization| {
            let ui = ui_handle.unwrap();
            let annuity_loan = credit_calculator::AnnuityLoan::from_loan_rate(
                loan.into(),
                annual_interest_rate.into(),
                annual_pay_off_rate.into(),
                annual_unscheduled_amortization.into(),
            );

            ui.set_credit_overview(annuity_loan.to_string().into());

            let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> =
                Rc::new(VecModel::default());

            for rate in annuity_loan.calculate_pay_off_plan() {
                let items = Rc::new(VecModel::default());

                items.push(
                    slint::format!(
                        "{:02} {}",
                        rate.payment_date.month(),
                        rate.payment_date.year_ce().1
                    )
                    .into(),
                );
                items.push(slint::format!("{:.2}€", rate.annuity).into());
                items.push(slint::format!("{:.2}€", rate.interest).into());
                items.push(slint::format!("{:.2}€", rate.amortization).into());
                items.push(slint::format!("{:.2}€", rate.unscheduled_amortization).into());
                items.push(
                    slint::format!("{}€", rate.residual_dept.round().separate_with_commas()).into(),
                );

                row_data.push(items.into());
            }

            ui.global::<TableViewAdapter>()
                .set_row_data(row_data.clone().into());
        }
    });

    ui.run()
}
