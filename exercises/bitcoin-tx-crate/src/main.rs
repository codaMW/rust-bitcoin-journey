mod tx_lib;

fn main() {
    println!("\n\n\n\n");

    println!(
        "------------------------------------TRANSACTION-1------------------------------------"
    );

    tx_lib::transaction_parser(
        "02000000000101402df457cc86b6b0c493eedd74495a0b164d836b11dc\
    f554cc5b07b3813f23480100000017160014b95eaa8558c86a1ec2d435ff2338362b6d42b\
    f77fdffffff02c0e1e400000000001976a914376cc71a571cc3b581b761aaf78bbc813837\
6bf788accc089a020000000017a914f6220280b0c90c0bcdf5a7877256cefabdb36d8c870\
2483045022100f2f8597c7d967e520ff17e3c94de341ba7b528baebe69da0e841ad035fea\
651d02206d290faf25cfbd6b7e87c1e3466fb2a4be5ef0a4cd97253523b6552b996396fa0\
12102900c508bef8f01ebfe77d34d16110dade2af5877cbe530d56e22c10097ba4d4f00000000",
    );

    println!("\n\n\n\n");

    println!(
        "------------------------------------TRANSACTION-2------------------------------------"
    );

    tx_lib::transaction_parser(
        "020000000001019d05f10a885d79047b2336aa77adb7550ad1a307fbe76c\
        ffd411d2684264069a000000001716001484159092e5ef9ba7251c44baa044c948547db682fdffffff01\
        7c5100000000000017a91455b3ef81ba2b59d5ba8e07af14e11f7c9a65ded98702483045022100f2287\
        f0f7c1c68ef2f050ca826b685060077ca2909788d3098df2d08b05c450802201ef6875be8a96b91764\
        0d6642fea3c6d62488048ee341667b8a26e708fe5477f012102b0c1f15593455ff25b1f382444d8abd\
        aede0015e1524c1a8dce8091672b5a28500000000",
    );

    println!("\n\n\n\n");
}
