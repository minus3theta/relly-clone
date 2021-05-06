use relly_clone::{
    buffer::BufferPool, buffer::BufferPoolManager, disk::DiskManager, disk::PageId,
    table::SimpleTable,
};

fn main() -> anyhow::Result<()> {
    let disk = DiskManager::open("simple.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let mut table = SimpleTable {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
    };
    table.create(&mut bufmgr)?;
    dbg!(&table);
    {
        let mut insert = |record| table.insert(&mut bufmgr, record);
        insert(&[b"z", b"Alice", b"Smith"])?;
        insert(&[b"x", b"Bob", b"Johnson"])?;
        insert(&[b"y", b"Charlie", b"Williams"])?;
        insert(&[b"w", b"Dave", b"Miller"])?;
        insert(&[b"v", b"Eve", b"Brown"])?;
    }

    bufmgr.flush()?;

    Ok(())
}
