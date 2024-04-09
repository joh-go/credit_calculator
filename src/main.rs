pub mod credit_calculator;

slint::include_modules!();

use chrono::Datelike;
use slint::{Model, StandardListViewItem, VecModel};
use std::rc::Rc;
use thousands::Separable;

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    ui.on_request_credit_plan_rates({
        let ui_handle = ui.as_weak();
        move |loan,
              annual_interest_rate,
              annual_pay_off_rate,
              annual_unscheduled_amortization,
              credit_id| {
            let ui = ui_handle.unwrap();
            let annuity_loan = credit_calculator::AnnuityLoan::from_loan_rate(
                loan.into(),
                annual_interest_rate.into(),
                annual_pay_off_rate.into(),
                annual_unscheduled_amortization.into(),
            );

            show_credit_plan(&annuity_loan, ui, credit_id);
        }
    });

    ui.on_request_credit_plan_annuity({
        let ui_handle = ui.as_weak();
        move |loan,
              annual_interest_rate,
              monthly_annuity,
              annual_unscheduled_amortization,
              credit_id| {
            let ui = ui_handle.unwrap();
            let annuity_loan = credit_calculator::AnnuityLoan::from_loan_annuity(
                loan.into(),
                annual_interest_rate.into(),
                monthly_annuity.into(),
                annual_unscheduled_amortization.into(),
            );

            show_credit_plan(&annuity_loan, ui, credit_id);
        }
    });

    ui.run()
}

fn show_credit_plan(annuity_loan: &credit_calculator::AnnuityLoan, ui: AppWindow, credit_id: i32) {
    let credit_overview = ui.get_credit_overview();

    credit_overview.set_row_data(
        credit_id.try_into().unwrap(),
        annuity_loan.to_string().into(),
    );

    let row_data: Rc<VecModel<slint::ModelRc<StandardListViewItem>>> = Rc::new(VecModel::default());

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
        items.push(slint::format!("{}€", rate.residual_dept.round().separate_with_commas()).into());

        row_data.push(items.into());
    }

    let row_datas = ui.get_row_datas();

    row_datas.set_row_data(credit_id.try_into().unwrap(), row_data.clone().into());
}
