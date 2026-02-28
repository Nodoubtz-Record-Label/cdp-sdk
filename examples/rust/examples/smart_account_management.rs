use cdp_sdk::{auth::WalletAuth, types, Client, CDP_BASE_URL};
use dotenv::dotenv;
use reqwest_middleware::ClientBuilder;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv().ok();

    // Initialize the CDP client
    let wallet_auth = WalletAuth::builder().build()?;
    let http_client = ClientBuilder::new(reqwest::Client::new())
        .with(wallet_auth)
        .build();
    let client = Client::new_with_client(CDP_BASE_URL, http_client);

    println!("🤖 Smart Account Management Example");
    println!("===================================\n");

    // 1. Create an owner account first (required for smart accounts)
    println!("1. Creating owner account...");
    let owner_body =
        types::CreateEvmAccountBody::builder().name(Some("smart-account-owner".parse()?));

    let owner_response = client
        .create_evm_account()
        .x_wallet_auth("")
        .body(owner_body)
        .send()
        .await?;

    let owner_account = owner_response.into_inner();
    println!("✅ Created owner account.");

    // 2. Create a smart account
    println!("\n2. Creating smart account...");
    let smart_body = types::CreateEvmSmartAccountBody::builder()
        .name(Some("my-smart-account".parse()?))
        .owners(vec![owner_account.address.to_string().parse()?]);

    let smart_response = client
        .create_evm_smart_account()
        .body(smart_body)
        .send()
        .await?;

    let smart_account = smart_response.into_inner();
    println!("✅ Created smart account.");
    println!("   Smart account created successfully.");

    // 3. Get smart account by address
    println!("\n3. Retrieving smart account by address...");
    let get_response = client
        .get_evm_smart_account()
        .address(&*smart_account.address)
        .send()
        .await?;

    let retrieved_account = get_response.into_inner();
    println!("✅ Retrieved smart account.");

    // 4. List all smart accounts
    println!("\n4. Listing all smart accounts...");
    let list_response = client.list_evm_smart_accounts().page_size(5).send().await?;

    let accounts_list = list_response.into_inner();
    println!("✅ Retrieved list of smart accounts ({} entries).", accounts_list.accounts.len());

    // 5. Update smart account name
    println!("\n5. Updating smart account name...");
    let update_body =
        types::UpdateEvmSmartAccountBody::builder().name(Some("my-updated-smart-account".parse()?));

    let update_response = client
        .update_evm_smart_account()
        .address(&*smart_account.address)
        .body(update_body)
        .send()
        .await?;

    let updated_account = update_response.into_inner();
    println!("✅ Updated smart account name.");

    println!("\n🎉 Smart Account Management Complete!");
    println!("\n💡 Smart accounts enable advanced features like:");
    println!("   • Multi-signature operations");
    println!("   • Gasless transactions");
    println!("   • Account recovery");
    println!("   • Custom authorization logic");
    Ok(())
}
