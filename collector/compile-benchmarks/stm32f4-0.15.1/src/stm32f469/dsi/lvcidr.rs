#[doc = "Register `LVCIDR` reader"]
pub struct R(crate::R<LVCIDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LVCIDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LVCIDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `LVCIDR` writer"]
pub struct W(crate::W<LVCIDR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<LVCIDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<LVCIDR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<LVCIDR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `VCID` reader - Virtual Channel ID"]
pub type VCID_R = crate::FieldReader<u8, u8>;
#[doc = "Field `VCID` writer - Virtual Channel ID"]
pub type VCID_W<'a, const O: u8> = crate::FieldWriter<'a, u32, LVCIDR_SPEC, u8, u8, 2, O>;
impl R {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&self) -> VCID_R {
        VCID_R::new((self.bits & 3) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1 - Virtual Channel ID"]
    #[inline(always)]
    pub fn vcid(&mut self) -> VCID_W<0> {
        VCID_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "DSI Host LTDC VCID Register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lvcidr](index.html) module"]
pub struct LVCIDR_SPEC;
impl crate::RegisterSpec for LVCIDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lvcidr::R](R) reader structure"]
impl crate::Readable for LVCIDR_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [lvcidr::W](W) writer structure"]
impl crate::Writable for LVCIDR_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets LVCIDR to value 0"]
impl crate::Resettable for LVCIDR_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
