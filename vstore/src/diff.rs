
use std::sync::Arc;
use crate::index::Index;
use keyvalue::KeyValue;
use valuepack::Pack;
use log::debug;
use crate::Error;

pub struct DiffIter {
    pub (crate) kv: Arc<Box<dyn KeyValue>>,
    pub (crate) a_idx: Index,
    pub (crate) b_idx: Index,

    a_pack: Option<Pack>,
    b_pack: Option<Pack>,
    pos: usize,
}

impl DiffIter {
    pub (crate) fn new(kv: Arc<Box<dyn KeyValue>>, a_idx: Index, b_idx: Index) -> Self {
        let d = DiffIter {
            a_idx,
            b_idx,
            kv,
            pos: 0,
            a_pack: None,
            b_pack: None,
        };
        d
    }

    fn next_part_diff(&mut self) -> Option<(usize, u16, u16)> {
        while self.pos < self.a_idx.len() {
            let a_part_ver = self.a_idx.get_prefix_version(self.pos);
            let b_part_ver = self.b_idx.get_prefix_version(self.pos);
            if a_part_ver == 0 && (a_part_ver == b_part_ver) {
                self.pos += 1;
            }else{
                let part = self.pos;
                self.pos += 1;
                return Some((part, a_part_ver, b_part_ver))
            }
        }
        None
    }

    fn load_pack(&self, part: usize, part_ver: u16) -> Result<Option<Pack>, Error> {
        if part == 0 {
            return Ok(None)
        }

        let part = part as u32;
        let part_key = part.to_be_bytes();

        match self.kv.get(part_ver, &part_key[..])? {
            None => Ok(None),
            Some(buf) => {
                Ok(Some(Pack::from_buf(&buf)?))
            }
        }
    }

    fn all_new(&self, p: &Pack) -> Vec<DiffItem> {
        let mut items = Vec::new();

        for (k,v) in p.map.iter() {
            items.push(DiffItem{
                key: k.clone(),
                diff_type: DiffType::New(v.1.clone()),
            });
        }

        items
    }

    fn all_delete(&self, p: &Pack) -> Vec<DiffItem> {
        let mut items = Vec::new();

        for (k,v) in p.map.iter() {
            items.push(DiffItem{
                key: k.clone(),
                diff_type: DiffType::Delete(v.0, v.1.clone()),
            });
        }

        items
    }

    fn diff_packs(&self, a: &Pack, b: &Pack) -> Vec<DiffItem> {
        let mut items = Vec::new();

        for (k, (b_ver, b_val)) in b.map.iter() {
            match a.map.get(k) {
                None => {
                    items.push(DiffItem{
                        key: k.clone(),
                        diff_type: DiffType::New(b_val.clone()),
                    });
                },
                Some((a_ver, a_val)) => {
                    if a_ver != b_ver {
                        items.push(DiffItem{
                            key: k.clone(),
                            diff_type: DiffType::Value(DiffValue{
                                a_ver: *a_ver,
                                b_ver: *b_ver,
                                a_val: a_val.clone(),
                                b_val: b_val.clone(),
                            })
                        });
                    }
                }
            }
        }

        for (k, (a_ver, a_val)) in a.map.iter() {
            match b.get(k) {
                Some(_) => {},
                None => {
                    items.push(DiffItem{
                        key: k.clone(),
                        diff_type: DiffType::Delete(*a_ver, a_val.clone())
                    });
                }
            }
        }

        items
    }

    fn process_packs(&self) -> Result<Vec<DiffItem>, Error> {
        match (&self.a_pack, &self.b_pack) {
            (None, Some(b_pack)) => {
                Ok(self.all_new(b_pack))
            },
            (Some(a_pack), None) => {
                Ok(self.all_delete(a_pack))
            },
            (Some(a_pack), Some(b_pack)) => {
               Ok(self.diff_packs(a_pack, b_pack))
            },
            (None, None) => {
                println!("Error: A & B not present");
                Err(Error::InvalidDiffState)
            },
        }
    }

    pub fn next(&mut self) -> Result<Option<Vec<DiffItem>>, Error> {
        if self.a_pack.is_none() && self.b_pack.is_none() {
            let part_diff_ver = self.next_part_diff();
            debug!("diff part version found: {:?}", part_diff_ver);

            if part_diff_ver.is_none() {
                return Ok(None);
            }

            let (part, a_part_ver, b_part_ver) = part_diff_ver.unwrap();

            self.a_pack = self.load_pack(part, a_part_ver)?;
            self.b_pack = self.load_pack(part, b_part_ver)?;
        }

        let items = self.process_packs()?;
        self.a_pack = None;
        self.b_pack = None;
        Ok(Some(items))
    }
}

pub struct DiffValue {
    pub a_ver: u16,
    pub b_ver: u16,
    pub a_val: Vec<u8>,
    pub b_val: Vec<u8>,
}

pub enum DiffType {
    New(Vec<u8>),
    Delete(u16, Vec<u8>),
    Value(DiffValue),
}

pub struct DiffItem {
    pub key: Vec<u8>,
    pub diff_type: DiffType,
}
