//! This is based on `bindgen` produced FFI binding for MKL's spblas
//! (sparse blas) interface.
use super::*;

extern "C" {
    pub fn mkl_scsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scscmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scscsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scoomv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scoosv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_scootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sdiamv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sdiasv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sdiagemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sdiasymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sdiatrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sskymv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sskysv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sbsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sbsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_sbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_sbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_sbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_cspblas_sbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn mkl_scsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scscmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scscsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scoomm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scoosm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sdiamm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sdiasm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sskysm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sskymm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sbsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sbsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSCMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSCSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCOOMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCOOSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SDIAMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SDIASV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SDIAGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SDIASYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SDIATRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SSKYMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SSKYSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SBSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        beta: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SBSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_SBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f32,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f32,
        y: *mut f32,
    );
}
extern "C" {
    pub fn MKL_SCSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSCMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSCSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCOOMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCOOSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SDIAMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SDIASM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SSKYSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SSKYMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        pntr: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SBSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f32,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SBSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f32,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f32,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f32,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcscmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcscsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcoomv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcoosv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dcootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_ddiamv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_ddiasv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_ddiagemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_ddiasymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_ddiatrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dskymv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dskysv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dbsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dbsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_cspblas_dbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn mkl_dcsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcscmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcscsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcoomm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcoosm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ddiamm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ddiasm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dskysm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dskymm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dbsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dbsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSCMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSCSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCOOMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCOOSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DDIAMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DDIASV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DDIAGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DDIASYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DDIATRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DSKYMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DSKYSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DBSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        beta: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DBSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_DBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const f64,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const f64,
        y: *mut f64,
    );
}
extern "C" {
    pub fn MKL_DCSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSCMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSCSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCOOMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCOOSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DDIAMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DDIASM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DSKYSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DSKYMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        pntr: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DBSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        beta: *const f64,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DBSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const f64,
        matdescra: *const ::std::os::raw::c_char,
        val: *const f64,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const f64,
        ldb: *const ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccscmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccscsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccoomv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccoosv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_ccootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cdiamv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cdiasv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cdiagemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cdiasymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cdiatrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cskymv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cskysv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cbsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cbsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_cbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_cbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_cspblas_cbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn mkl_ccsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccscmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccscsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccoomm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccoosm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cdiamm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cdiasm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cskysm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cskymm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cbsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cbsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSCMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSCSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCOOMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCOOSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CDIAMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CDIASV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CDIAGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CDIASYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CDIATRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSKYMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSKYSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CBSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        beta: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CBSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_CBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex8,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    );
}
extern "C" {
    pub fn MKL_CCSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSCMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSCSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCOOMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCOOSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CDIAMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CDIASM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CSKYSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CSKYMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CBSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CBSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex8,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex8,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex8,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcscmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcscsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcoomv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcoosv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcoogemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcoosymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zcootrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zdiamv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zdiasv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zdiagemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zdiasymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zdiatrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zskymv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zskysv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zbsrmv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zbsrsv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zbsrgemv(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zbsrsymv(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_cspblas_zbsrtrsv(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn mkl_zcsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcscmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcscsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcoomm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcoosm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zdiamm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zdiasm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zskysm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zskymm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zbsrmm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zbsrsm(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSCMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSCSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCOOMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCOOSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCOOGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCOOSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZCOOTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZDIAMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZDIASV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZDIAGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZDIASYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZDIATRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZSKYMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZSKYSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZBSRMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        beta: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZBSRSV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZBSRGEMV(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZBSRSYMV(
        uplo: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_CSPBLAS_ZBSRTRSV(
        uplo: *const ::std::os::raw::c_char,
        transa: *const ::std::os::raw::c_char,
        diag: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        a: *const MKL_Complex16,
        ia: *const ::std::os::raw::c_int,
        ja: *const ::std::os::raw::c_int,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    );
}
extern "C" {
    pub fn MKL_ZCSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSCMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSCSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCOOMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCOOSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        rowind: *const ::std::os::raw::c_int,
        colind: *const ::std::os::raw::c_int,
        nnz: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZDIAMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZDIASM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        lval: *const ::std::os::raw::c_int,
        idiag: *const ::std::os::raw::c_int,
        ndiag: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZSKYSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZSKYMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        pntr: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZBSRMM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZBSRSM(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        lb: *const ::std::os::raw::c_int,
        alpha: *const MKL_Complex16,
        matdescra: *const ::std::os::raw::c_char,
        val: *const MKL_Complex16,
        indx: *const ::std::os::raw::c_int,
        pntrb: *const ::std::os::raw::c_int,
        pntre: *const ::std::os::raw::c_int,
        b: *const MKL_Complex16,
        ldb: *const ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *const ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrbsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut f64,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrcoo(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut f64,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ddnscsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut f64,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrcsc(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut f64,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrdia(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut f64,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut f64,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrsky(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut f64,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrbsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut f32,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrcoo(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut f32,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_sdnscsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut f32,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrcsc(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut f32,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrdia(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut f32,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut f32,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrsky(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut f32,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrbsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut MKL_Complex8,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrcoo(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut MKL_Complex8,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_cdnscsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut MKL_Complex8,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrcsc(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut MKL_Complex8,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrdia(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut MKL_Complex8,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut MKL_Complex8,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrsky(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut MKL_Complex8,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrbsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut MKL_Complex16,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrcoo(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut MKL_Complex16,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zdnscsr(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut MKL_Complex16,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrcsc(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut MKL_Complex16,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrdia(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut MKL_Complex16,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut MKL_Complex16,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrsky(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut MKL_Complex16,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRBSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut f64,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRCOO(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut f64,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DDNSCSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut f64,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRCSC(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut f64,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRDIA(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut f64,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut f64,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRSKY(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f64,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut f64,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRBSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut f32,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRCOO(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut f32,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SDNSCSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut f32,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRCSC(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut f32,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRDIA(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut f32,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut f32,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRSKY(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut f32,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut f32,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRBSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut MKL_Complex8,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRCOO(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut MKL_Complex8,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CDNSCSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut MKL_Complex8,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRCSC(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut MKL_Complex8,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRDIA(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut MKL_Complex8,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut MKL_Complex8,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRSKY(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex8,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut MKL_Complex8,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRBSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        mblk: *const ::std::os::raw::c_int,
        ldAbsr: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        Absr: *mut MKL_Complex16,
        AJB: *mut ::std::os::raw::c_int,
        AIB: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRCOO(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJR: *mut ::std::os::raw::c_int,
        AIR: *mut ::std::os::raw::c_int,
        nnz: *mut ::std::os::raw::c_int,
        Acoo: *mut MKL_Complex16,
        ir: *mut ::std::os::raw::c_int,
        jc: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZDNSCSR(
        job: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Adns: *mut MKL_Complex16,
        lda: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ: *mut ::std::os::raw::c_int,
        AI: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRCSC(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Acsc: *mut MKL_Complex16,
        AJ1: *mut ::std::os::raw::c_int,
        AI1: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRDIA(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Adia: *mut MKL_Complex16,
        ndiag: *const ::std::os::raw::c_int,
        distance: *mut ::std::os::raw::c_int,
        idiag: *mut ::std::os::raw::c_int,
        Acsr_rem: *mut MKL_Complex16,
        AJ0_rem: *mut ::std::os::raw::c_int,
        AI0_rem: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRSKY(
        job: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        Acsr: *mut MKL_Complex16,
        AJ0: *mut ::std::os::raw::c_int,
        AI0: *mut ::std::os::raw::c_int,
        Asky: *mut MKL_Complex16,
        pointers: *mut ::std::os::raw::c_int,
        info: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrmultcsr(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsrmultd(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_dcsradd(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const f64,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrmultcsr(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsrmultd(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_scsradd(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const f32,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrmultcsr(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsrmultd(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_ccsradd(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrmultcsr(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsrmultd(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn mkl_zcsradd(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRMULTCSR(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRMULTD(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_DCSRADD(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut f64,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const f64,
        b: *mut f64,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f64,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRMULTCSR(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRMULTD(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_SCSRADD(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut f32,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const f32,
        b: *mut f32,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut f32,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRMULTCSR(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRMULTD(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_CCSRADD(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex8,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const MKL_Complex8,
        b: *mut MKL_Complex8,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex8,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRMULTCSR(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRMULTD(
        transa: *const ::std::os::raw::c_char,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        k: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        ldc: *mut ::std::os::raw::c_int,
    );
}
extern "C" {
    pub fn MKL_ZCSRADD(
        transa: *const ::std::os::raw::c_char,
        job: *const ::std::os::raw::c_int,
        sort: *const ::std::os::raw::c_int,
        m: *const ::std::os::raw::c_int,
        n: *const ::std::os::raw::c_int,
        a: *mut MKL_Complex16,
        ja: *mut ::std::os::raw::c_int,
        ia: *mut ::std::os::raw::c_int,
        beta: *const MKL_Complex16,
        b: *mut MKL_Complex16,
        jb: *mut ::std::os::raw::c_int,
        ib: *mut ::std::os::raw::c_int,
        c: *mut MKL_Complex16,
        jc: *mut ::std::os::raw::c_int,
        ic: *mut ::std::os::raw::c_int,
        nnzmax: *const ::std::os::raw::c_int,
        ierr: *mut ::std::os::raw::c_int,
    );
}
pub const sparse_status_t_SPARSE_STATUS_SUCCESS: sparse_status_t = 0;
pub const sparse_status_t_SPARSE_STATUS_NOT_INITIALIZED: sparse_status_t = 1;
pub const sparse_status_t_SPARSE_STATUS_ALLOC_FAILED: sparse_status_t = 2;
pub const sparse_status_t_SPARSE_STATUS_INVALID_VALUE: sparse_status_t = 3;
pub const sparse_status_t_SPARSE_STATUS_EXECUTION_FAILED: sparse_status_t = 4;
pub const sparse_status_t_SPARSE_STATUS_INTERNAL_ERROR: sparse_status_t = 5;
pub const sparse_status_t_SPARSE_STATUS_NOT_SUPPORTED: sparse_status_t = 6;
pub type sparse_status_t = ::std::os::raw::c_uint;
pub const sparse_operation_t_SPARSE_OPERATION_NON_TRANSPOSE: sparse_operation_t = 10;
pub const sparse_operation_t_SPARSE_OPERATION_TRANSPOSE: sparse_operation_t = 11;
pub const sparse_operation_t_SPARSE_OPERATION_CONJUGATE_TRANSPOSE: sparse_operation_t = 12;
pub type sparse_operation_t = ::std::os::raw::c_uint;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_GENERAL: sparse_matrix_type_t = 20;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_SYMMETRIC: sparse_matrix_type_t = 21;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_HERMITIAN: sparse_matrix_type_t = 22;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_TRIANGULAR: sparse_matrix_type_t = 23;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_DIAGONAL: sparse_matrix_type_t = 24;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_BLOCK_TRIANGULAR: sparse_matrix_type_t = 25;
pub const sparse_matrix_type_t_SPARSE_MATRIX_TYPE_BLOCK_DIAGONAL: sparse_matrix_type_t = 26;
pub type sparse_matrix_type_t = ::std::os::raw::c_uint;
pub const sparse_index_base_t_SPARSE_INDEX_BASE_ZERO: sparse_index_base_t = 0;
pub const sparse_index_base_t_SPARSE_INDEX_BASE_ONE: sparse_index_base_t = 1;
pub type sparse_index_base_t = ::std::os::raw::c_uint;
pub const sparse_fill_mode_t_SPARSE_FILL_MODE_LOWER: sparse_fill_mode_t = 40;
pub const sparse_fill_mode_t_SPARSE_FILL_MODE_UPPER: sparse_fill_mode_t = 41;
pub const sparse_fill_mode_t_SPARSE_FILL_MODE_FULL: sparse_fill_mode_t = 42;
pub type sparse_fill_mode_t = ::std::os::raw::c_uint;
pub const sparse_diag_type_t_SPARSE_DIAG_NON_UNIT: sparse_diag_type_t = 50;
pub const sparse_diag_type_t_SPARSE_DIAG_UNIT: sparse_diag_type_t = 51;
pub type sparse_diag_type_t = ::std::os::raw::c_uint;
pub const sparse_layout_t_SPARSE_LAYOUT_ROW_MAJOR: sparse_layout_t = 101;
pub const sparse_layout_t_SPARSE_LAYOUT_COLUMN_MAJOR: sparse_layout_t = 102;
pub type sparse_layout_t = ::std::os::raw::c_uint;
pub const verbose_mode_t_SPARSE_VERBOSE_OFF: verbose_mode_t = 70;
pub const verbose_mode_t_SPARSE_VERBOSE_BASIC: verbose_mode_t = 71;
pub const verbose_mode_t_SPARSE_VERBOSE_EXTENDED: verbose_mode_t = 72;
pub type verbose_mode_t = ::std::os::raw::c_uint;
pub const sparse_memory_usage_t_SPARSE_MEMORY_NONE: sparse_memory_usage_t = 80;
pub const sparse_memory_usage_t_SPARSE_MEMORY_AGGRESSIVE: sparse_memory_usage_t = 81;
pub type sparse_memory_usage_t = ::std::os::raw::c_uint;
pub const sparse_request_t_SPARSE_STAGE_FULL_MULT: sparse_request_t = 90;
pub const sparse_request_t_SPARSE_STAGE_NNZ_COUNT: sparse_request_t = 91;
pub const sparse_request_t_SPARSE_STAGE_FINALIZE_MULT: sparse_request_t = 92;
pub const sparse_request_t_SPARSE_STAGE_FULL_MULT_NO_VAL: sparse_request_t = 93;
pub const sparse_request_t_SPARSE_STAGE_FINALIZE_MULT_NO_VAL: sparse_request_t = 94;
pub type sparse_request_t = ::std::os::raw::c_uint;
#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct sparse_matrix {
    _unused: [u8; 0],
}
pub type sparse_matrix_t = *mut sparse_matrix;
#[repr(C)]
#[derive(Debug, Copy, Clone, Hash, PartialOrd, Ord, PartialEq, Eq)]
pub struct matrix_descr {
    pub type_: sparse_matrix_type_t,
    pub mode: sparse_fill_mode_t,
    pub diag: sparse_diag_type_t,
}
#[test]
fn bindgen_test_layout_matrix_descr() {
    assert_eq!(
        ::core::mem::size_of::<matrix_descr>(),
        12usize,
        concat!("Size of: ", stringify!(matrix_descr))
    );
    assert_eq!(
        ::core::mem::align_of::<matrix_descr>(),
        4usize,
        concat!("Alignment of ", stringify!(matrix_descr))
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<matrix_descr>())).type_ as *const _ as usize },
        0usize,
        concat!(
            "Offset of field: ",
            stringify!(matrix_descr),
            "::",
            stringify!(type_)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<matrix_descr>())).mode as *const _ as usize },
        4usize,
        concat!(
            "Offset of field: ",
            stringify!(matrix_descr),
            "::",
            stringify!(mode)
        )
    );
    assert_eq!(
        unsafe { &(*(::core::ptr::null::<matrix_descr>())).diag as *const _ as usize },
        8usize,
        concat!(
            "Offset of field: ",
            stringify!(matrix_descr),
            "::",
            stringify!(diag)
        )
    );
}
impl Default for matrix_descr {
    fn default() -> Self {
        unsafe { ::core::mem::zeroed() }
    }
}
extern "C" {
    pub fn mkl_sparse_s_create_coo(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_create_coo(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_create_coo(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_create_coo(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        nnz: ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_create_csr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_create_csr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_create_csr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_create_csr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_create_csc(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        cols_start: *mut ::std::os::raw::c_int,
        cols_end: *mut ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        values: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_create_csc(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        cols_start: *mut ::std::os::raw::c_int,
        cols_end: *mut ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        values: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_create_csc(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        cols_start: *mut ::std::os::raw::c_int,
        cols_end: *mut ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_create_csc(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        cols_start: *mut ::std::os::raw::c_int,
        cols_end: *mut ::std::os::raw::c_int,
        row_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_create_bsr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        block_layout: sparse_layout_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        block_size: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_create_bsr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        block_layout: sparse_layout_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        block_size: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_create_bsr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        block_layout: sparse_layout_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        block_size: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_create_bsr(
        A: *mut sparse_matrix_t,
        indexing: sparse_index_base_t,
        block_layout: sparse_layout_t,
        rows: ::std::os::raw::c_int,
        cols: ::std::os::raw::c_int,
        block_size: ::std::os::raw::c_int,
        rows_start: *mut ::std::os::raw::c_int,
        rows_end: *mut ::std::os::raw::c_int,
        col_indx: *mut ::std::os::raw::c_int,
        values: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_copy(
        source: sparse_matrix_t,
        descr: matrix_descr,
        dest: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_destroy(A: sparse_matrix_t) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_get_error_info(
        A: sparse_matrix_t,
        info: *mut ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_convert_csr(
        source: sparse_matrix_t,
        operation: sparse_operation_t,
        dest: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_convert_bsr(
        source: sparse_matrix_t,
        block_size: ::std::os::raw::c_int,
        block_layout: sparse_layout_t,
        operation: sparse_operation_t,
        dest: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_export_bsr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        block_layout: *mut sparse_layout_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        block_size: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_export_bsr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        block_layout: *mut sparse_layout_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        block_size: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_export_bsr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        block_layout: *mut sparse_layout_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        block_size: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_export_bsr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        block_layout: *mut sparse_layout_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        block_size: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_export_csr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_export_csr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_export_csr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_export_csr(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        rows_start: *mut *mut ::std::os::raw::c_int,
        rows_end: *mut *mut ::std::os::raw::c_int,
        col_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_export_csc(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        cols_start: *mut *mut ::std::os::raw::c_int,
        cols_end: *mut *mut ::std::os::raw::c_int,
        row_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_export_csc(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        cols_start: *mut *mut ::std::os::raw::c_int,
        cols_end: *mut *mut ::std::os::raw::c_int,
        row_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_export_csc(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        cols_start: *mut *mut ::std::os::raw::c_int,
        cols_end: *mut *mut ::std::os::raw::c_int,
        row_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_export_csc(
        source: sparse_matrix_t,
        indexing: *mut sparse_index_base_t,
        rows: *mut ::std::os::raw::c_int,
        cols: *mut ::std::os::raw::c_int,
        cols_start: *mut *mut ::std::os::raw::c_int,
        cols_end: *mut *mut ::std::os::raw::c_int,
        row_indx: *mut *mut ::std::os::raw::c_int,
        values: *mut *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_set_value(
        A: sparse_matrix_t,
        row: ::std::os::raw::c_int,
        col: ::std::os::raw::c_int,
        value: f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_set_value(
        A: sparse_matrix_t,
        row: ::std::os::raw::c_int,
        col: ::std::os::raw::c_int,
        value: f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_set_value(
        A: sparse_matrix_t,
        row: ::std::os::raw::c_int,
        col: ::std::os::raw::c_int,
        value: MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_set_value(
        A: sparse_matrix_t,
        row: ::std::os::raw::c_int,
        col: ::std::os::raw::c_int,
        value: MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_update_values(
        A: sparse_matrix_t,
        nvalues: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        indy: *const ::std::os::raw::c_int,
        values: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_update_values(
        A: sparse_matrix_t,
        nvalues: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        indy: *const ::std::os::raw::c_int,
        values: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_update_values(
        A: sparse_matrix_t,
        nvalues: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        indy: *const ::std::os::raw::c_int,
        values: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_update_values(
        A: sparse_matrix_t,
        nvalues: ::std::os::raw::c_int,
        indx: *const ::std::os::raw::c_int,
        indy: *const ::std::os::raw::c_int,
        values: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_verbose_mode(verbose: verbose_mode_t) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_mv_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        expected_calls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_dotmv_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        expectedCalls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_mm_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        dense_matrix_size: ::std::os::raw::c_int,
        expected_calls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_sv_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        expected_calls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_sm_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        dense_matrix_size: ::std::os::raw::c_int,
        expected_calls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_symgs_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        expected_calls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_lu_smoother_hint(
        A: sparse_matrix_t,
        operation: sparse_operation_t,
        descr: matrix_descr,
        expectedCalls: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_set_memory_hint(
        A: sparse_matrix_t,
        policy: sparse_memory_usage_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_optimize(A: sparse_matrix_t) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_order(A: sparse_matrix_t) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_mv(
        operation: sparse_operation_t,
        alpha: f32,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f32,
        beta: f32,
        y: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_mv(
        operation: sparse_operation_t,
        alpha: f64,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f64,
        beta: f64,
        y: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_mv(
        operation: sparse_operation_t,
        alpha: MKL_Complex8,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex8,
        beta: MKL_Complex8,
        y: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_mv(
        operation: sparse_operation_t,
        alpha: MKL_Complex16,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex16,
        beta: MKL_Complex16,
        y: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_dotmv(
        transA: sparse_operation_t,
        alpha: f32,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f32,
        beta: f32,
        y: *mut f32,
        d: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_dotmv(
        transA: sparse_operation_t,
        alpha: f64,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f64,
        beta: f64,
        y: *mut f64,
        d: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_dotmv(
        transA: sparse_operation_t,
        alpha: MKL_Complex8,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex8,
        beta: MKL_Complex8,
        y: *mut MKL_Complex8,
        d: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_dotmv(
        transA: sparse_operation_t,
        alpha: MKL_Complex16,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex16,
        beta: MKL_Complex16,
        y: *mut MKL_Complex16,
        d: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_trsv(
        operation: sparse_operation_t,
        alpha: f32,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f32,
        y: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_trsv(
        operation: sparse_operation_t,
        alpha: f64,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const f64,
        y: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_trsv(
        operation: sparse_operation_t,
        alpha: MKL_Complex8,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex8,
        y: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_trsv(
        operation: sparse_operation_t,
        alpha: MKL_Complex16,
        A: sparse_matrix_t,
        descr: matrix_descr,
        x: *const MKL_Complex16,
        y: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_symgs(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: f32,
        b: *const f32,
        x: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_symgs(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: f64,
        b: *const f64,
        x: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_symgs(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: MKL_Complex8,
        b: *const MKL_Complex8,
        x: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_symgs(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: MKL_Complex16,
        b: *const MKL_Complex16,
        x: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_symgs_mv(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: f32,
        b: *const f32,
        x: *mut f32,
        y: *mut f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_symgs_mv(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: f64,
        b: *const f64,
        x: *mut f64,
        y: *mut f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_symgs_mv(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: MKL_Complex8,
        b: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        y: *mut MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_symgs_mv(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        alpha: MKL_Complex16,
        b: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        y: *mut MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_lu_smoother(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        diag: *const f32,
        approx_diag_inverse: *const f32,
        x: *mut f32,
        rhs: *const f32,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_lu_smoother(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        diag: *const f64,
        approx_diag_inverse: *const f64,
        x: *mut f64,
        rhs: *const f64,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_lu_smoother(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        diag: *const MKL_Complex8,
        approx_diag_inverse: *const MKL_Complex8,
        x: *mut MKL_Complex8,
        rhs: *const MKL_Complex8,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_lu_smoother(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        descr: matrix_descr,
        diag: *const MKL_Complex16,
        approx_diag_inverse: *const MKL_Complex16,
        x: *mut MKL_Complex16,
        rhs: *const MKL_Complex16,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_mm(
        operation: sparse_operation_t,
        alpha: f32,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const f32,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        beta: f32,
        y: *mut f32,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_mm(
        operation: sparse_operation_t,
        alpha: f64,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const f64,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        beta: f64,
        y: *mut f64,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_mm(
        operation: sparse_operation_t,
        alpha: MKL_Complex8,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const MKL_Complex8,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        beta: MKL_Complex8,
        y: *mut MKL_Complex8,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_mm(
        operation: sparse_operation_t,
        alpha: MKL_Complex16,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const MKL_Complex16,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        beta: MKL_Complex16,
        y: *mut MKL_Complex16,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_trsm(
        operation: sparse_operation_t,
        alpha: f32,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const f32,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        y: *mut f32,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_trsm(
        operation: sparse_operation_t,
        alpha: f64,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const f64,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        y: *mut f64,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_trsm(
        operation: sparse_operation_t,
        alpha: MKL_Complex8,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const MKL_Complex8,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        y: *mut MKL_Complex8,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_trsm(
        operation: sparse_operation_t,
        alpha: MKL_Complex16,
        A: sparse_matrix_t,
        descr: matrix_descr,
        layout: sparse_layout_t,
        x: *const MKL_Complex16,
        columns: ::std::os::raw::c_int,
        ldx: ::std::os::raw::c_int,
        y: *mut MKL_Complex16,
        ldy: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_add(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: f32,
        B: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_add(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: f64,
        B: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_add(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: MKL_Complex8,
        B: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_add(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: MKL_Complex16,
        B: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_spmm(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_sp2m(
        transA: sparse_operation_t,
        descrA: matrix_descr,
        A: sparse_matrix_t,
        transB: sparse_operation_t,
        descrB: matrix_descr,
        B: sparse_matrix_t,
        request: sparse_request_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_syrk(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        C: *mut sparse_matrix_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_sypr(
        transA: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        descrB: matrix_descr,
        C: *mut sparse_matrix_t,
        request: sparse_request_t,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_syprd(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        B: *const f32,
        layoutB: sparse_layout_t,
        ldb: ::std::os::raw::c_int,
        alpha: f32,
        beta: f32,
        C: *mut f32,
        layoutC: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_syprd(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        B: *const f64,
        layoutB: sparse_layout_t,
        ldb: ::std::os::raw::c_int,
        alpha: f64,
        beta: f64,
        C: *mut f64,
        layoutC: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_syprd(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        B: *const MKL_Complex8,
        layoutB: sparse_layout_t,
        ldb: ::std::os::raw::c_int,
        alpha: MKL_Complex8,
        beta: MKL_Complex8,
        C: *mut MKL_Complex8,
        layoutC: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_syprd(
        op: sparse_operation_t,
        A: sparse_matrix_t,
        B: *const MKL_Complex16,
        layoutB: sparse_layout_t,
        ldb: ::std::os::raw::c_int,
        alpha: MKL_Complex16,
        beta: MKL_Complex16,
        C: *mut MKL_Complex16,
        layoutC: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_spmmd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        layout: sparse_layout_t,
        C: *mut f32,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_spmmd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        layout: sparse_layout_t,
        C: *mut f64,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_spmmd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        layout: sparse_layout_t,
        C: *mut MKL_Complex8,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_spmmd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        B: sparse_matrix_t,
        layout: sparse_layout_t,
        C: *mut MKL_Complex16,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_sp2md(
        transA: sparse_operation_t,
        descrA: matrix_descr,
        A: sparse_matrix_t,
        transB: sparse_operation_t,
        descrB: matrix_descr,
        B: sparse_matrix_t,
        alpha: f32,
        beta: f32,
        C: *mut f32,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_sp2md(
        transA: sparse_operation_t,
        descrA: matrix_descr,
        A: sparse_matrix_t,
        transB: sparse_operation_t,
        descrB: matrix_descr,
        B: sparse_matrix_t,
        alpha: f64,
        beta: f64,
        C: *mut f64,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_sp2md(
        transA: sparse_operation_t,
        descrA: matrix_descr,
        A: sparse_matrix_t,
        transB: sparse_operation_t,
        descrB: matrix_descr,
        B: sparse_matrix_t,
        alpha: MKL_Complex8,
        beta: MKL_Complex8,
        C: *mut MKL_Complex8,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_sp2md(
        transA: sparse_operation_t,
        descrA: matrix_descr,
        A: sparse_matrix_t,
        transB: sparse_operation_t,
        descrB: matrix_descr,
        B: sparse_matrix_t,
        alpha: MKL_Complex16,
        beta: MKL_Complex16,
        C: *mut MKL_Complex16,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_s_syrkd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: f32,
        beta: f32,
        C: *mut f32,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_d_syrkd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: f64,
        beta: f64,
        C: *mut f64,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_c_syrkd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: MKL_Complex8,
        beta: MKL_Complex8,
        C: *mut MKL_Complex8,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}
extern "C" {
    pub fn mkl_sparse_z_syrkd(
        operation: sparse_operation_t,
        A: sparse_matrix_t,
        alpha: MKL_Complex16,
        beta: MKL_Complex16,
        C: *mut MKL_Complex16,
        layout: sparse_layout_t,
        ldc: ::std::os::raw::c_int,
    ) -> sparse_status_t;
}

#[cfg(test)]
mod tests {
    use super::*;
    use sprs::{prod::mul_acc_mat_vec_csr, CompressedStorage, CsMatViewI};

    #[test]
    fn csr_mv() {
        use std::os::raw::{c_char, c_int};

        let indptr: &[u32] = &[0, 3, 3, 5, 6, 7];
        let indices: &[u32] = &[1, 2, 3, 2, 3, 4, 4];
        let data: &[f64] = &[
            0.75672424, 0.1649078, 0.30140296, 0.10358244, 0.6283315, 0.39244208, 0.57202407,
        ];

        let mat =
            CsMatViewI::new_view(CompressedStorage::CSR, (5, 5), indptr, indices, data).unwrap();
        let slice: &[f64] = &[0.1, 0.2, -0.1, 0.3, 0.9];
        let mut res_vec = vec![0., 0., 0., 0., 0.];

        mul_acc_mat_vec_csr(mat, slice, &mut res_vec);

        let mut mkl_ret = vec![0., 0., 0., 0., 0.];

        let t: u8 = b'N';
        let m: i32 = res_vec.len() as i32;
        assert_eq!(std::mem::size_of::<c_int>(), std::mem::size_of::<i32>());
        unsafe {
            mkl_cspblas_dcsrgemv(
                &t as *const u8 as *const c_char,
                &m as *const i32,
                mat.data().as_ptr(),
                mat.indptr().as_ptr() as *const c_int,
                mat.indices().as_ptr() as *const c_int,
                slice.as_ptr(),
                mkl_ret.as_mut_ptr(),
            );
        }

        assert!(res_vec
            .iter()
            .zip(mkl_ret.iter())
            .all(|(x, y)| approx::abs_diff_eq!(*x, *y, epsilon = f64::EPSILON)));
    }
}
