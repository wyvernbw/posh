use sealed::sealed;

use crate::{
    gl::{Sampler2dBinding, Texture2dBinding, UniformBufferBinding, VertexBuffer},
    program_def::VertexInputRate,
    sl, Gl, Numeric, Sl,
};

use super::{
    FragmentInterface, FragmentInterfaceVisitor, Primitive, ResourceInterface, Uniform, Vertex,
    VertexInterface, VertexInterfaceVisitor,
};

#[sealed]
impl super::Domain for Gl {
    type Scalar<T: Primitive> = T;
    type Vec2<T: Primitive> = mint::Vector2<T>;
    type Vec3<T: Primitive> = mint::Vector3<T>;
    type Vec4<T: Primitive> = mint::Vector4<T>;

    type Bool = bool;
    type F32 = f32;
    type I32 = i32;
    type U32 = u32;
}

// Uniform

unsafe impl Uniform<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Uniform<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Uniform<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Uniform<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl<T: Primitive> Uniform<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;
}

unsafe impl<T: Primitive> Uniform<Gl> for mint::Vector3<T> {
    type InGl = T::Vec3;
    type InSl = sl::Vec3<T>;
}

unsafe impl<T: Primitive> Uniform<Gl> for mint::Vector4<T> {
    type InGl = T::Vec4;
    type InSl = sl::Vec4<T>;
}

// Vertex

unsafe impl Vertex<Gl> for bool {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Vertex<Gl> for f32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Vertex<Gl> for i32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl Vertex<Gl> for u32 {
    type InGl = Self;
    type InSl = sl::Scalar<Self>;
}

unsafe impl<T: Primitive> Vertex<Gl> for mint::Vector2<T> {
    type InGl = T::Vec2;
    type InSl = sl::Vec2<T>;
}

unsafe impl<T: Primitive> Vertex<Gl> for mint::Vector3<T> {
    type InGl = T::Vec3;
    type InSl = sl::Vec3<T>;
}

unsafe impl<T: Primitive> Vertex<Gl> for mint::Vector4<T> {
    type InGl = T::Vec4;
    type InSl = sl::Vec4<T>;
}

// VertexInterface

#[sealed]
impl super::VertexDomain for Gl {
    type Vertex<V: Vertex<Sl>> = VertexBuffer<V>;
}

unsafe impl<V: Vertex<Sl>> VertexInterface<Gl> for VertexBuffer<V> {
    type InGl = Self;
    type InSl = V::InSl;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl VertexInterfaceVisitor<'a, Gl>) {
        visitor.accept(path, VertexInputRate::Vertex, self)
    }
}

#[sealed]
impl<V: Vertex<Sl>> super::VertexInterfaceField<Gl> for VertexBuffer<V> {}

// ResourceInterface

#[sealed]
impl super::ResourceDomain for Gl {
    type Sampler2d<T: Numeric> = Sampler2dBinding<T>;
    type Uniform<U: Uniform<Sl, InSl = U>> = UniformBufferBinding<U>;
    type Compose<R: ResourceInterface<Sl>> = R::InGl;
}

unsafe impl<T: Numeric> ResourceInterface<Gl> for Sampler2dBinding<T> {
    type InGl = Self;
    type InSl = sl::Sampler2d<T>;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<'a, Gl>) {
        visitor.accept_sampler2d(path, self);
    }
}

unsafe impl<U: Uniform<Sl, InSl = U>> ResourceInterface<Gl> for UniformBufferBinding<U> {
    type InGl = Self;
    type InSl = U;

    fn visit<'a>(&'a self, path: &str, visitor: &mut impl super::ResourceInterfaceVisitor<'a, Gl>) {
        visitor.accept_uniform::<U::InSl>(path, self);
    }
}

// FragmentInterface

#[sealed]
impl super::FragmentDomain for Gl {
    type Attachment = Texture2dBinding;
}

unsafe impl FragmentInterface<Gl> for Texture2dBinding {
    type InGl = Self;
    type InSl = sl::Vec4<f32>;

    fn visit(&self, path: &str, visitor: &mut impl FragmentInterfaceVisitor<Gl>) {
        visitor.accept(path, self);
    }
}
