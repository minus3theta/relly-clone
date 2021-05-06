use relly_clone::{
    buffer::{BufferPool, BufferPoolManager},
    disk::{DiskManager, PageId},
    table::{Table, UniqueIndex},
};

fn main() -> anyhow::Result<()> {
    let disk = DiskManager::open("table.rly")?;
    let pool = BufferPool::new(10);
    let mut bufmgr = BufferPoolManager::new(disk, pool);

    let mut table = Table {
        meta_page_id: PageId::INVALID_PAGE_ID,
        num_key_elems: 1,
        unique_indices: vec![UniqueIndex {
            meta_page_id: PageId::INVALID_PAGE_ID,
            skey: vec![2],
        }],
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
