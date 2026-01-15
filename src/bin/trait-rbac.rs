/*
Without Trait, would be hard and not effective whenever there are new roles.
*/

enum _Role {
  Admin,
  Staff,
  Guest,
}

struct _User {
  username: String,
  role: _Role,
}

fn _can_update(user: &_User) -> bool {
  match user.role {
    _Role::Admin => true,
    _Role::Staff => true,
    _Role::Guest => false,
  }
}

fn _can_remove(user: &_User) -> bool {
  match user.role {
    _Role::Admin => true,
    _Role::Staff => false,
    _Role::Guest => false,
  }
}

/*
With Trait, any future update new role will be more simple.
*/

// 1. Buat Kontrak
trait AccessPermission {
  // Default: Ga semua orang bisa hapus data (aman!)
  fn can_delete_data(&self) -> bool {
    false
  }

  // Wajib diimplementasikan oleh setiap Role
  fn can_edit_data(&self) -> bool;

  // Name role untuk keperluan print
  fn role_name(&self) -> &str;
}

// 2. Buat Struct untuk masing-masing Role
struct Admin;
struct Staff;
struct Guest;

// 3. Implementasikan Trait untuk Admin
impl AccessPermission for Admin {
  fn role_name(&self) -> &str {
    "Administrator"
  }
  fn can_edit_data(&self) -> bool {
    true
  }
  fn can_delete_data(&self) -> bool {
    true
  }
}

// 4. Implementasikan Trait untuk Staff
impl AccessPermission for Staff {
  fn role_name(&self) -> &str {
    "Staff Operasional"
  }
  fn can_edit_data(&self) -> bool {
    true
  }
  // can_delete_data otomatis false karena tidak di-override
}

// 5. Implementasikan Trait untuk Guest
impl AccessPermission for Guest {
  fn role_name(&self) -> &str {
    "General Guest"
  }
  fn can_edit_data(&self) -> bool {
    false
  }
}

fn execute_update_database<T: AccessPermission>(user: &T) {
  println!(
    "\n=== Attempting to UPDATE DATA as [{}] ===",
    user.role_name()
  );

  if user.can_edit_data() {
    println!("✅ ACCESS GRANTED: Database successfully updated.")
  } else {
    println!("❌ ACCESS DENIED: You do not have permission")
  }
}

fn execute_remove_database<T: AccessPermission>(user: &T) {
  println!(
    "\n=== Attempting to DELETE DATA as [{}] ===",
    user.role_name()
  );

  if user.can_delete_data() {
    println!("✅ ACCESS GRANTED: Database successfully deleted.")
  } else {
    println!("❌ ACCESS DENIED: You do not have permission")
  }
}

fn main() {
  let user_admin = Admin;
  let user_staff = Staff;
  let user_guest = Guest;

  // Panggil fungsi yg sama dgn tipe data berbeda
  // Update
  execute_update_database(&user_admin);
  execute_update_database(&user_staff);
  execute_update_database(&user_guest);
  // Remove
  execute_remove_database(&user_admin);
  execute_remove_database(&user_staff);
  execute_remove_database(&user_guest);
}

/*
  So, if in the next 3 months there are new roles (Manager, Finance Operator),
  just create new structs and give the access permission without need to touch the existed roles.
  It will avoid the break code or create potentially new bug.
*/
