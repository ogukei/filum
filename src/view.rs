

use super::context::{Context};
use super::buffer::{Buffer};

use super::error::Result;
use std::sync::Arc;
use std::marker::PhantomData;


#[macro_export]
macro_rules! bindings {
    ($v0:expr,) => {
        $crate::BufferLayout::new_0($v0)
    };
    ($v0:expr, $v1:expr,) => {
        $crate::BufferLayout::new_1($v0, $v1)
    };
    ($v0:expr, $v1:expr, $v2:expr,) => {
        $crate::BufferLayout::new_2($v0, $v1, $v2)
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr,) => {
        $crate::BufferLayout::new_3($v0, $v1, $v2, $v3)
    };
    ($v0:expr, $v1:expr, $v2:expr, $v3:expr, $( $vv:expr ),*) => {
        $crate::BufferLayout::new(vec![$v0.size(), $v1.size(), $v2.size(), $v3.size(), $($vv.size()),*])
    };
}

#[macro_export]
macro_rules! binding_array {
    ($x:ty, $y:expr) => {
        $crate::BindingVariant::array($crate::BindingArray::<$x>::new($y))
    };
}

#[macro_export]
macro_rules! binding_value {
    ($x:ty) => {
        $crate::BindingVariant::value($crate::BindingValue::<$x>::new())
    };
}

pub struct BufferLayout<T0, T1 = (), T2 = (), T3 = ()> {
    bindings: (T0, T1, T2, T3),
    size: usize,
    entries: Vec<usize>,
}

impl<T0, T1, T2, T3> BufferLayout<T0, T1, T2, T3> {
    pub fn new(entries: Vec<usize>) -> BufferLayout<()> {
        BufferLayout { 
            bindings: ((), (), (), ()),
            size: entries.iter().sum(),
            entries: entries,
        }
    }
}

impl<T0> BufferLayout<BindingVariant<T0>> {
    pub fn new_0(v0: BindingVariant<T0>) -> Self {
        let entries = vec![v0.size()];
        BufferLayout { 
            bindings: (v0, (), (), ()),
            size: entries.iter().sum(),
            entries: entries,
        }
    }
}

impl<T0, T1> BufferLayout<BindingVariant<T0>, BindingVariant<T1>> {
    pub fn new_1(v0: BindingVariant<T0>, v1: BindingVariant<T1>) -> Self {
        let entries = vec![v0.size(), v1.size()];
        BufferLayout { 
            bindings: (v0, v1, (), ()),
            size: entries.iter().sum(),
            entries: entries,
        }
    }
}

impl<T0, T1, T2> BufferLayout<BindingVariant<T0>, BindingVariant<T1>, BindingVariant<T2>> {
    pub fn new_2(v0: BindingVariant<T0>, v1: BindingVariant<T1>, v2: BindingVariant<T2>) -> Self {
        let entries = vec![v0.size(), v1.size(), v2.size()];
        BufferLayout { 
            bindings: (v0, v1, v2, ()),
            size: entries.iter().sum(),
            entries: entries,
        }
    }
}

impl<T0, T1, T2, T3> BufferLayout<BindingVariant<T0>, BindingVariant<T1>, BindingVariant<T2>, BindingVariant<T3>> {
    pub fn new_3(v0: BindingVariant<T0>, v1: BindingVariant<T1>, v2: BindingVariant<T2>, v3: BindingVariant<T3>) -> Self {
        let entries = vec![v0.size(), v1.size(), v2.size(), v3.size()];
        BufferLayout { 
            bindings: (v0, v1, v2, v3),
            size: entries.iter().sum(),
            entries: entries,
        }
    }
}

pub struct BindingArray<T> {
    data: PhantomData<T>,
    count: usize,
}

impl<T> BindingArray<T> {
    pub fn new(count: usize) -> Self {
        BindingArray { data: PhantomData, count }
    }

    pub fn count(&self) -> usize {
        self.count
    }
}

pub struct BindingValue<T> {
    data: PhantomData<T>,
}

impl<T> BindingValue<T> {
    pub fn new() -> Self {
        BindingValue { data: PhantomData }
    }
}

pub struct BindingVariant<T> {
    variant: T,
    size: usize,
}

impl<T> BindingVariant<BindingValue<T>> {
    pub fn value(variant: BindingValue<T>) -> Self {
        BindingVariant { 
            variant, 
            size: std::mem::size_of::<T>(),
        }
    }
}

impl<T> BindingVariant<BindingArray<T>> {
    pub fn array(variant: BindingArray<T>) -> Self {
        let size = std::mem::size_of::<T>() * variant.count();
        BindingVariant { 
            variant, 
            size: size,
        }
    }
}

impl<T> BindingVariant<T> {
    pub fn size(&self) -> usize {
        self.size
    }
}

pub struct BufferViewBuilder<'a, LayoutType> {
    layout: LayoutType,
    context: &'a Arc<Context>,
}

impl<'a> BufferViewBuilder<'a, ()> {
    pub fn new(context: &'a Arc<Context>) -> Self {
        BufferViewBuilder {
            layout: (),
            context: context,
        }
    }
}

impl<'a> BufferViewBuilder<'a, ()> {
    pub fn layout<LayoutType>(self, layout: LayoutType) -> BufferViewBuilder<'a, LayoutType> {
        BufferViewBuilder {
            layout: layout,
            context: self.context,
        }
    }

    pub fn bind_array<ItemType>(self, count: usize) -> BufferViewBuilder<'a, BufferLayout<BindingVariant<BindingArray<ItemType>>>> {
        BufferViewBuilder {
            layout: bindings!(binding_array!(ItemType, count),),
            context: self.context,
        }
    }

    pub fn bind_value<ValueType>(self) -> BufferViewBuilder<'a, BufferLayout<BindingVariant<BindingValue<ValueType>>>> {
        BufferViewBuilder {
            layout: bindings!(binding_value!(ValueType),),
            context: self.context,
        }
    }
}

impl<'a, T0, T1, T2, T3> BufferViewBuilder<'a, BufferLayout<T0, T1, T2, T3>> {
    pub fn build(self) -> Result<Arc<BufferView<BufferLayout<T0, T1, T2, T3>>>> {
        BufferView::new(self.layout, self.context)
    }
}

pub struct BufferView<LayoutType> {
    data: PhantomData<LayoutType>,
    buffer: Arc<Buffer>,
}

impl<LayoutType> BufferView<LayoutType> {
    // returns underlying buffer, type erasure.
    #[inline]
    pub fn buffer(&self) -> &Arc<Buffer> {
        &self.buffer
    }
}

impl<T0, T1, T2, T3> BufferView<BufferLayout<T0, T1, T2, T3>> {
    pub fn new(layout: BufferLayout<T0, T1, T2, T3>, context: &Arc<Context>) -> Result<Arc<Self>> {
        let buffer = Buffer::new(context, layout.entries);
        let view = BufferView { 
            data: PhantomData,
            buffer: buffer,
        };
        Ok(Arc::new(view))
    }
}

impl<T0> BufferView<BufferLayout<BindingVariant<T0>>> {
    pub fn binding(&self) -> Arc<BufferBindingView<T0>> {
        BufferBindingView::new(0, &self.buffer)
    }
}

impl<T0, T1, T2, T3> BufferView<BufferLayout<BindingVariant<T0>, T1, T2, T3>> {
    pub fn first_binding(&self) -> Arc<BufferBindingView<T0>> {
        BufferBindingView::new(0, &self.buffer)
    }
}

impl<T0, T1, T2, T3> BufferView<BufferLayout<T0, BindingVariant<T1>, T2, T3>> {
    pub fn second_binding(&self) -> Arc<BufferBindingView<T1>> {
        BufferBindingView::new(1, &self.buffer)
    }
}

impl<T0, T1, T2, T3> BufferView<BufferLayout<T0, BindingVariant<T1>, BindingVariant<T2>, T3>> {
    pub fn third_binding(&self) -> Arc<BufferBindingView<T2>> {
        BufferBindingView::new(2, &self.buffer)
    }
}

impl<T0, T1, T2, T3> BufferView<BufferLayout<T0, BindingVariant<T1>, BindingVariant<T2>, BindingVariant<T3>>> {
    pub fn fourth_binding(&self) -> Arc<BufferBindingView<T3>> {
        BufferBindingView::new(3, &self.buffer)
    }
}

impl<LayoutType> BufferView<BufferLayout<LayoutType>> {
    pub fn nth_binding(&self, index: usize) -> Option<Arc<BufferBindingView>> {
        let buffer = &self.buffer;
        if index < buffer.region_count() {
            Some(BufferBindingView::new(index, buffer))
        } else {
            None
        }
    }
}

pub struct BufferBindingView<VariantType = ()> {
    data: PhantomData<VariantType>,
    region_index: usize,
    buffer: Arc<Buffer>,
}

impl<VariantType> BufferBindingView<VariantType> {
    pub fn new(region_index: usize, buffer: &Arc<Buffer>) -> Arc<Self> {
        let binding = BufferBindingView { 
            data: PhantomData, 
            region_index, 
            buffer: Arc::clone(buffer),
        };
        Arc::new(binding)
    }
}

impl<ItemType> BufferBindingView<BindingArray<ItemType>> {
    pub fn update_array(&self, array: &[ItemType]) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.write_region(self.region_index, array);
    }

    pub fn fetch_array(&self, array: &mut [ItemType]) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.read_region(self.region_index, array);
    }
}

impl<ValueType> BufferBindingView<BindingValue<ValueType>> {
    pub fn update_value(&self, value: &ValueType) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.write_region(self.region_index, value);
    }

    pub fn fetch_value(&self, value: &mut ValueType) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.read_region(self.region_index, value);
    }
}

impl BufferBindingView<()> {
    pub fn update<DataType: ?Sized>(&self, value: &DataType) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.write_region(self.region_index, value);
    }

    pub fn fetch<DataType: ?Sized>(&self, value: &mut DataType) {
        let staging_buffer = self.buffer.staging_buffer();
        staging_buffer.read_region(self.region_index, value);
    }
}
