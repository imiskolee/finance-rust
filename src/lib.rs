//! finance-rust is a simple common finance mathematics in rust.
//! Its cloning form [http://financejs.org/] .
//! The create includes some finance mathematics theory & formula (eg: CAGR,CI,LR,PI,AM etc.).
///


/*
Copyright (c) <2016> <copyright pow2n10>

Permission is hereby granted, free of charge, to any person obtaining a copy of this software and associated documentation files (the "Software"), to deal in the Software without restriction, including without limitation the rights to use, copy, modify, merge, publish, distribute, sublicense, and/or sell copies of the Software, and to permit persons to whom the Software is furnished to do so, subject to the following conditions:

The above copyright notice and this permission notice shall be included in all copies or substantial portions of the Software.

THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY, FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE SOFTWARE.
*/



/// In business, amortization refers to spreading payments over multiple periods.
/// The term is used for two separate processes: amortization of loans and assets.
/// It also refers to allocating the cost of an intangible asset over a period of time.
/// see definition: https://en.wikipedia.org/wiki/Amortization_%28business%29
pub fn am(principal:f64,rate:f64,period:f64,
          year_or_month:bool,pay_at_beginning:bool) -> f64 {

    let rate_perperiod = rate / 12_f64 / 100_f64;

    let numerator;
    let denominator;

    let build_num = |num:f64| -> f64 {
        if pay_at_beginning {
            num - 1_f64
        }else{
            rate_perperiod * ((1_f64 + rate_perperiod).powf(num))
        }
    };

    if year_or_month {
        numerator = build_num(period);
        denominator = ((1_f64 + rate_perperiod).powf(period)) - 1_f64

    }else{
        numerator = build_num(period * 12_f64);
        denominator = ((1_f64 + rate_perperiod).powf(period*12_f64)) - 1_f64;
    }

    ((principal * (numerator / denominator ) * 100_f64).round()) / 100_f64
}

/// Compound annual growth rate (CAGR) is a business and investing specific term for the geometric progression ratio that provides a constant rate of return over the time period.
/// [1][2] CAGR is not an accounting term, but it is often used to describe some element of the business, for example revenue, units delivered, registered users, etc.
/// CAGR dampens the effect of volatility of periodic returns that can render arithmetic means irrelevant.
/// It is particularly useful to compare growth rates from various data sets of common domain such as revenue growth of companies in the same industry.
/// see definition: http://pages.uoregon.edu/rgp/PPPM613/class8a.htm
pub fn cagr(beging_val:f64,end_val:f64,num_of_periods:f64) -> f64 {

    (((end_val / beging_val).powf(1_f64 / num_of_periods) -1_f64)*10000_f64).round() / 100_f64
}

/// he addition of interest to the principal sum of a loan or deposit is called compounding.
/// Compound interest is interest on interest.
/// It is the result of reinvesting interest, rather than paying it out, so that interest in the next period is then earned on the principal sum plus previously-accumulated interest.
/// Compound interest is standard in finance and economics.
/// see definition: https://en.wikipedia.org/wiki/Compound_interest
pub fn ci(rate:f64,num_of_compoundings:f64,principal:f64,num_of_periods:f64) -> f64 {
    ((principal
     *
     (
         (1_f64 +
          ((rate / 100_f64) / num_of_compoundings)
          )
          .powf(num_of_compoundings * num_of_periods))
    ) * 100_f64).round() / 100_f64
}

/// Discounting is a financial mechanism in which a debtor obtains the right to delay payments to a creditor, for a defined period of time, in exchange for a charge or fee.
/// Essentially, the party that owes money in the present purchases the right to delay the payment until some future date.
/// The discount, or charge, is the difference (expressed as a difference in the same units (absolute) or in percentage terms (relative), or as a ratio) between the original amount owed in the present and the amount that has to be paid in the future to settle the debt.
/// see definition: https://en.wikipedia.org/wiki/Discounting
pub fn df(rate:f64,num_of_periods:i32) -> Vec<f64>{

    let mut dfs:Vec<f64> = Vec::new();

    for i in 1..num_of_periods {
        dfs.push(
            ((1_f64/(1_f64 + rate/100_f64).powi(i-1)) * 1000_f64).ceil()
                / 1000_f64
        );
    }

    dfs
}

/// Future value is the value of an asset at a specific date.
/// It measures the nominal future sum of money that a given sum of money is "worth" at a specified time in the future assuming a certain interest rate, or more generally, rate of return; it is the present value multiplied by the accumulation function.
/// The value does not include corrections for inflation or other factors that affect the true value of money in the future.
/// This is used in time value of money calculations.
/// see definition: https://en.wikipedia.org/wiki/Future_value
pub fn fv(rate:f64,cf0:f64,num_of_period:f64) -> f64 {
    (cf0 * (1_f64 + rate/100_f64).powf(num_of_period) * 100_f64 ).round() / 100_f64
}


/// The internal rate of return (IRR) or external rate of return (ERR) is a method of calculating rate of return.
/// The term internal refers to the fact that its calculation does not incorporate environmental factors (e.g., the interest rate or inflation).
/// see definition: https://en.wikipedia.org/wiki/Internal_rate_of_return
pub fn irr(cfs:&[f64]) -> f64{

    let mut num_of_tries = 1;

    let npv_res  = |rate:f64| -> f64 {
        num_of_tries = num_of_tries + 1;
        if num_of_tries > 1000 {
            panic!("IRR can't find a result");
        }
        npv(rate,cfs)
    };
    (seek_zero(npv_res) * 100_f64).round() / 100_f64
}


/// In finance, the net present value (NPV) or net present worth (NPW)[1] is a measurement of the profitability of an undertaking that is calculated by subtracting the present values (PV) of cash outflows (including initial cost) from the present values of cash inflows over a period of time.
/// Incoming and outgoing cash flows can also be described as benefit and cost cash flows, respectively.
/// see definition: https://en.wikipedia.org/wiki/Net_present_value
pub fn npv(rate:f64,vals:&[f64]) -> f64 {
    let r = 1_f64 + rate / 100_f64;
    ((vals.iter().enumerate().fold(0_f64,|p,(i,&v)|{
        match i {
            0 => v,
            _ => {
                p + v / (r).powi(i as i32)
            }
        }
    }
    ))
    * 100_f64).round() / 100_f64
}

/// Payback period in capital budgeting refers to the period of time required to recoup the funds expended in an investment, or to reach the break-even point.
/// see definition: https://en.wikipedia.org/wiki/Payback_period
pub fn pp(num_of_periods:f64,cfs:&[f64]) -> f64{

    if cfs.len() < 2 {
        panic!("slice:cfs can't lt 2")
    }

    if num_of_periods == 0_f64 {
        return (cfs[0] / cfs[1]).abs()
    }

    let mut years = 1_f64;

    let mut cumulative_cash_flow = cfs[0];

    for v in cfs[1..].iter() {
        cumulative_cash_flow = cumulative_cash_flow + v;
         if cumulative_cash_flow > 0_f64 {
                years = years + (cumulative_cash_flow - v) / v;
             return  (years * 100_f64).round() /100_f64
            }else{
                years = years + 1_f64;
            }
    }
    (years * 100_f64).round() /100_f64
}

/// In economics, present value, also known as present discounted value, is the value of an expected income stream determined as of the date of valuation.
/// The present value is always less than or equal to the future value because money has interest-earning potential, a characteristic referred to as the time value of money, except during times of negative interest rates, when the present value will be more than the future value.
/// Time value can be described with the simplified phrase, "A dollar today is worth more than a dollar tomorrow".
/// Here, 'worth more' means that its value is greater. A dollar today is worth more than a dollar tomorrow because the dollar can be invested and earn a day's worth of interest, making the total accumulate to a value more than a dollar by tomorrow.
/// Interest can be compared to rent.[2] Just as rent is paid to a landlord by a tenant, without the ownership of the asset being transferred, interest is paid to a lender by a borrower who gains access to the money for a time before paying it back.
/// By letting the borrower have access to the money, the lender has sacrificed the exchange value of this money, and is compensated for it in the form of interest.
/// The initial amount of the borrowed funds (the present value) is less than the total amount of money paid to the lender.
/// see definition: https://en.wikipedia.org/wiki/Present_value
pub fn pv(rate:f64,cf1:f64) -> f64{
    (cf1 / (1_f64 + rate/(100_f64))).round()
}

/// Profitability index (PI), also known as profit investment ratio (PIR) and value investment ratio (VIR), is the ratio of payoff to investment of a proposed project.
/// It is a useful tool for ranking projects because it allows you to quantify the amount of value created per unit of investment.
/// see definition: https://en.wikipedia.org/wiki/Profitability_index
pub fn pi(rate:f64,cfs:&[f64]) -> f64{

    if cfs.len() < 2 {
        panic!("slice:cfs.len() can't lt 2");
    }

    (((cfs[1..].iter().enumerate().fold(0_f64,|p,(i,&v)|{
       p + ( v * (1_f64 / ((1_f64 + rate/100_f64).powi((i+1) as i32))))
    })) / cfs[0].abs()) * 100_f64).round() / 100_f64
}

/// Return on Investment (ROI) is the benefit to an investor resulting from an investment of some resource.
/// A high ROI means the investment gains compare favorably to investment cost.
/// As a performance measure, ROI is used to evaluate the efficiency of an investment or to compare the efficiency of a number of different investments.
/// In purely economic terms, it is one way of considering profits in relation to capital invested.
/// see definition: https://en.wikipedia.org/wiki/Return_on_investment
pub fn roi(cf0:f64,earnings:f64) -> f64{
    (((earnings - cf0.abs()) / cf0.abs() * 100_f64) * 100_f64).round() / 100_f64
}

/// In finance, leverage (sometimes referred to as gearing in the United Kingdom and Australia) is any technique involving the use of borrowed funds in the purchase of an asset, with the expectation that the after tax income from the asset and asset price appreciation will exceed the borrowing cost.
/// Normally, the finance provider would set a limit on how much risk it is prepared to take and will set a limit on how much leverage it will permit, and would require the acquired asset to be provided as collateral security for the loan.
/// see definition: https://en.wikipedia.org/wiki/Leverage_(finance)
pub fn lr(total_liabilities:f64,total_debts:f64,total_income:f64) -> f64 {
    (total_liabilities + total_debts) / total_income
}


/// In finance, the rule of 72, the rule of 70 and the rule of 69.3 are methods for estimating an investment's doubling time. The rule number (e.g., 72) is divided by the interest percentage per period to obtain the approximate number of periods (usually years) required for doubling
/// see definition: https://en.wikipedia.org/wiki/Rule_of_72
pub fn r72(rate:f64) -> f64 {
    72_f64 /rate
}

/// The weighted average cost of capital (WACC) is the rate that a company is expected to pay on average to all its security holders to finance its assets.
/// The WACC is commonly referred to as the firm’s cost of capital.
/// Importantly, it is dictated by the external market and not by management.
/// The WACC represents the minimum return that a company must earn on an existing asset base to satisfy its creditors, owners, and other providers of capital, or they will invest elsewhere.
/// see definition: https://en.wikipedia.org/wiki/Weighted_average_cost_of_capital
pub fn wacc(market_val_of_equity:f64,market_val_of_debt:f64,cost_of_equity:f64,cost_of_debt:f64,tax_rate:f64) -> f64{

    (((
        (market_val_of_equity / (market_val_of_equity + market_val_of_debt)) * cost_of_equity / 100_f64)
     +
      ((
          (market_val_of_debt /(market_val_of_equity + market_val_of_debt))* cost_of_debt/100_f64)
      *
      (1_f64 - tax_rate/100_f64))
    )* 1000_f64).round() / 10_f64
}

fn seek_zero<F>(mut f:F) -> f64 where  F:FnMut(f64) -> f64 {

    let mut x = 1.0_64;

    loop {
        x =  if f(x) > 0_f64{
            x + 1_f64
        }else{
            break
        }
    }

    loop {
        x = if f(x) < 0_f64 {
           x - 0.01_f64
        }else{
            break
        }
    }
    
    x
}


#[cfg(test)]
    mod tests {
    use super::*;
    #[test]
    fn test_am() {
        assert_eq!(am(20000_f64, 7.5_f64, 5_f64, false,false),400.76);
    }
    #[test]
    fn test_cagr() {
        assert_eq!(cagr(10000_f64, 19500_f64, 3_f64),24.93);
    }

    #[test]
    fn test_ci() {
        assert_eq!(ci(4.3_f64,4_f64,1500_f64,6_f64),1938.84);
    }

    #[test]
    fn test_df() {
        assert_eq!(df(10_f64,6),vec![1_f64, 0.91, 0.827, 0.752, 0.684]);
    }

    #[test]
    fn test_fv(){
        assert_eq!(fv(0.5_f64,1000_f64,12_f64),1061.68);
    }

    #[test]
    fn test_npv() {
        assert_eq!(npv(10_f64,&[ -500000_f64, 200000_f64, 300000_f64, 200000_f64]),80015.03);
    }
    #[test]
    fn test_irr() {
        assert_eq!(irr(&[-500000_f64, 200000_f64, 300000_f64, 200000_f64]),18.82);
    }

    #[test]
    fn test_lr() {
        assert_eq!(lr(25_f64, 10_f64, 20_f64),1.75);
    }

    #[test]
    fn test_pp() {
        assert_eq!(pp(0_f64,&[-105_f64,25_f64]),4.2);
        assert_eq!(pp(5_f64,&[-50_f64, 10_f64, 13_f64, 16_f64, 19_f64, 22_f64]),3.42);
    }
    
    #[test]
    fn test_pi() {
        assert_eq!(pi(10_f64,&[ -40000_f64,18000_f64, 12000_f64, 10000_f64, 9000_f64, 6000_f64]),1.09);
    }

    #[test]
    fn test_roi() {
        assert_eq!(roi(-55000_f64,60000_f64),9.09);
    }

    #[test]
    fn test_r72(){
        assert_eq!(r72(10_f64),7.2);
    }

    #[test]
    fn test_wacc() {
        assert_eq!(wacc(600000_f64, 400000_f64, 6_f64, 5_f64, 35_f64),4.9);
    }

}
