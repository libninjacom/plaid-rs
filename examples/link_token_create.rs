use plaid::PlaidClient;
use plaid::model::*;
use plaid::request::LinkTokenCreateRequired;
#[tokio::main]
async fn main() {
    let client = PlaidClient::from_env();
    let args = LinkTokenCreateRequired {
        user: LinkTokenCreateRequestUser {
            email_address_verified_time: Some(
                "your email address verified time".to_owned(),
            ),
            id_number: Some(UserIdNumber {
                type_: "your type".to_owned(),
                value: "your value".to_owned(),
            }),
            address: Some(UserAddress {
                street: "your street".to_owned(),
                street2: Some("your street 2".to_owned()),
                city: "your city".to_owned(),
                region: "your region".to_owned(),
                country: "your country".to_owned(),
                postal_code: "your postal code".to_owned(),
            }),
            legal_name: Some("your legal name".to_owned()),
            ssn: Some("your ssn".to_owned()),
            date_of_birth: Some("your date of birth".to_owned()),
            client_user_id: "your client user id".to_owned(),
            phone_number_verified_time: Some(
                "your phone number verified time".to_owned(),
            ),
            name: Some(UserName {
                given_name: "your given name".to_owned(),
                family_name: "your family name".to_owned(),
            }),
            phone_number: Some("your phone number".to_owned()),
            email_address: Some("your email address".to_owned()),
        },
        language: "your language",
        client_name: "your client name",
        country_codes: &["your country codes"],
    };
    let response = client
        .link_token_create(args)
        .products(&["your products"])
        .additional_consented_products(&["your additional consented products"])
        .webhook("your webhook")
        .access_token("your access token")
        .link_customization_name("your link customization name")
        .redirect_uri("your redirect uri")
        .android_package_name("your android package name")
        .institution_data(LinkTokenCreateInstitutionData {
            routing_number: Some("your routing number".to_owned()),
        })
        .account_filters(LinkTokenAccountFilters {
            depository: Some(DepositoryFilter {
                account_subtypes: vec!["your account subtypes".to_owned()],
            }),
            credit: Some(CreditFilter {
                account_subtypes: vec!["your account subtypes".to_owned()],
            }),
            investment: Some(InvestmentFilter {
                account_subtypes: vec!["your account subtypes".to_owned()],
            }),
            loan: Some(LoanFilter {
                account_subtypes: vec!["your account subtypes".to_owned()],
            }),
        })
        .eu_config(LinkTokenEuConfig {
            headless: Some(true),
        })
        .institution_id("your institution id")
        .payment_initiation(LinkTokenCreateRequestPaymentInitiation {
            payment_id: "your payment id".to_owned(),
            consent_id: Some("your consent id".to_owned()),
        })
        .deposit_switch(LinkTokenCreateRequestDepositSwitch {
            deposit_switch_id: "your deposit switch id".to_owned(),
        })
        .income_verification(LinkTokenCreateRequestIncomeVerification {
            access_tokens: Some(vec!["your access tokens".to_owned()]),
            income_source_types: Some(vec!["your income source types".to_owned()]),
            stated_income_sources: Some(
                vec![
                    LinkTokenCreateRequestUserStatedIncomeSource { pay_type :
                    Some("your pay type".to_owned()), employer : Some("your employer"
                    .to_owned()), category : Some("your category".to_owned()),
                    pay_per_cycle : Some(1.0), pay_annual : Some(1.0), pay_frequency :
                    Some("your pay frequency".to_owned()) }
                ],
            ),
            payroll_income: Some(LinkTokenCreateRequestIncomeVerificationPayrollIncome {
                is_update_mode: Some(true),
                flow_types: Some(vec!["your flow types".to_owned()]),
            }),
            asset_report_id: Some("your asset report id".to_owned()),
            bank_income: Some(LinkTokenCreateRequestIncomeVerificationBankIncome {
                enable_multiple_items: Some(true),
                days_requested: Some(1),
            }),
            income_verification_id: Some("your income verification id".to_owned()),
            precheck_id: Some("your precheck id".to_owned()),
        })
        .auth(LinkTokenCreateRequestAuth {
            same_day_microdeposits_enabled: Some(true),
            flow_type: Some("your flow type".to_owned()),
            automated_microdeposits_enabled: Some(true),
            auth_type_select_enabled: Some(true),
            instant_match_enabled: Some(true),
        })
        .transfer(LinkTokenCreateRequestTransfer {
            payment_profile_id: Some("your payment profile id".to_owned()),
            intent_id: Some("your intent id".to_owned()),
        })
        .update(LinkTokenCreateRequestUpdate {
            account_selection_enabled: Some(true),
        })
        .identity_verification(LinkTokenCreateRequestIdentityVerification {
            gave_consent: Some(true),
            template_id: "your template id".to_owned(),
            consent: Some(true),
        })
        .user_token("your user token")
        .send()
        .await
        .unwrap();
    println!("{:#?}", response);
}
