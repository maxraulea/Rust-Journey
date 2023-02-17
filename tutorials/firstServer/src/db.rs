use mysql::*;

pub fn db_connection() -> PooledConn{
    let url = "mysql://root:hello_db@localhost:3306/mysql";
    let pool = Pool::new(url)
    .expect("an error ocurrded");

    return pool.get_conn().expect("connection failed");
}