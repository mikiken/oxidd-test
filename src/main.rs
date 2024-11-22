use oxidd::bdd::BDDFunction;
use oxidd::util::AllocResult;
use oxidd::BooleanFunction;
use oxidd::ManagerRef;

fn main() -> AllocResult<()> {
    let manager_ref = oxidd::bdd::new_manager(1024, 1024, 1);
    // 外部入力変数
    let (x1, x2, cin) = manager_ref.with_manager_exclusive(|manager| {
        (
            BDDFunction::new_var(manager).unwrap(),
            BDDFunction::new_var(manager).unwrap(),
            BDDFunction::new_var(manager).unwrap(),
        )
    });

    // 論理関数を定義
    let sum: BDDFunction = x1.xor(&x2)?.xor(&cin)?;
    let cout_orig: BDDFunction = x1.and(&x2)?.or(&x1.xor(&x2)?.and(&cin)?)?;
    let cout_spec: BDDFunction = x1.and(&x2)?.xor(&x1.xor(&x2)?.and(&cin)?)?;

    // 論理関数が充足可能か判定
    println!("sum.satisfiable()       : {}", sum.satisfiable()); // true
    println!("cout_orig.satisfiable() : {}", cout_orig.satisfiable()); // true
    println!("cout_spec.satisfiable() : {}", cout_spec.satisfiable()); // true

    // BDDが等価であるか判定
    println!("cout_orig == cout_spec  : {}", cout_orig == cout_spec); // true
    println!("cout_orig == sum        : {}", cout_orig == sum); // false

    Ok(())
}
