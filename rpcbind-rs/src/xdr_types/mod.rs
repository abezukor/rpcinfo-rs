pub mod port_mapper;
pub mod rpcbind;

trait LinkedList: Sized {
    type Value;

    fn make_elem(val: Self::Value) -> Self;
    fn set_next(&mut self, next: Box<Self>);
    fn get_next(&mut self) -> &mut Vec<Box<Self>>;
}

macro_rules! rpc_linked_list {
    ($tp:ty, $value:ty, $val:ident, $next:ident) => {
        impl LinkedList for $tp {
            type Value = $value;

            fn make_elem(val: Self::Value) -> Self {
                Self {
                    $val: val,
                    $next: Vec::new(),
                }
            }

            fn set_next(&mut self, next: Box<Self>) {
                self.$next.push(next);
            }

            fn get_next(&mut self) -> &mut Vec<Box<Self>> {
                &mut self.$next
            }
        }
    };
}
rpc_linked_list!(rpcbind::RPList, rpcbind::RPCB, rpcb_map, rpcb_next);
rpc_linked_list!(port_mapper::PMapList, port_mapper::Mapping, map, next);

#[allow(private_bounds)]
pub trait CreateList: LinkedList {
    #[allow(private_interfaces)]
    fn create_list<L: Iterator<Item = Self::Value>>(mut elements: L) -> Option<Self> {
        let first = elements.next()?;
        let mut raw = Self::make_elem(first);

        let mut current = &mut raw;
        for elem in elements {
            let raw = Box::new(Self::make_elem(elem));
            current.set_next(raw);
            current = &mut current.get_next()[0];
        }

        Some(raw)
    }
}

impl<L: LinkedList> CreateList for L {}
