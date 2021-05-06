use relly_clone::{
    btree::BTree,
    buffer::{BufferPool, BufferPoolManager},
    disk::DiskManager,
};

fn main() -> anyhow::Result<()> {
    let disk = DiskManager::open("test.btr")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let btree = BTree::create(&mut bufmgr)?;
    btree.insert(&mut bufmgr, b"Kanagawa", b"Yokohama")?;
    btree.insert(&mut bufmgr, b"Osaka", b"Osaka")?;
    btree.insert(&mut bufmgr, b"Aichi", b"Nagoya")?;

    bufmgr.flush()?;

    Ok(())
}
