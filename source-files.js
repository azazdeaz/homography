var N = null;var sourcesIndex = {};
sourcesIndex["approx"] = {"name":"","files":["abs_diff_eq.rs","lib.rs","macros.rs","relative_eq.rs","ulps_eq.rs"]};
sourcesIndex["bytemuck"] = {"name":"","files":["contiguous.rs","lib.rs","offset_of.rs","pod.rs","transparent.rs","zeroable.rs"]};
sourcesIndex["convert_case"] = {"name":"","files":["case.rs","lib.rs","words.rs"]};
sourcesIndex["cv_core"] = {"name":"","files":["camera.rs","keypoint.rs","lib.rs","matches.rs","point.rs","pose.rs","so3.rs","triangulation.rs"]};
sourcesIndex["derive_more"] = {"name":"","files":["add_assign_like.rs","add_helpers.rs","add_like.rs","as_mut.rs","as_ref.rs","constructor.rs","deref.rs","deref_mut.rs","display.rs","error.rs","from.rs","from_str.rs","index.rs","index_mut.rs","into.rs","into_iterator.rs","is_variant.rs","lib.rs","mul_assign_like.rs","mul_helpers.rs","mul_like.rs","not_like.rs","parsing.rs","sum_like.rs","try_into.rs","unwrap.rs","utils.rs"]};
sourcesIndex["either"] = {"name":"","files":["lib.rs"]};
sourcesIndex["eyre"] = {"name":"","files":["backtrace.rs","chain.rs","context.rs","error.rs","fmt.rs","kind.rs","lib.rs","macros.rs","wrapper.rs"]};
sourcesIndex["homography"] = {"name":"","files":["homography.rs","lib.rs"]};
sourcesIndex["indenter"] = {"name":"","files":["lib.rs"]};
sourcesIndex["itertools"] = {"name":"","dirs":[{"name":"adaptors","files":["coalesce.rs","map.rs","mod.rs","multi_product.rs"]}],"files":["combinations.rs","combinations_with_replacement.rs","concat_impl.rs","cons_tuples_impl.rs","diff.rs","duplicates_impl.rs","either_or_both.rs","exactly_one_err.rs","flatten_ok.rs","format.rs","free.rs","group_map.rs","groupbylazy.rs","grouping_map.rs","impl_macros.rs","intersperse.rs","k_smallest.rs","kmerge_impl.rs","lazy_buffer.rs","lib.rs","merge_join.rs","minmax.rs","multipeek_impl.rs","pad_tail.rs","peek_nth.rs","peeking_take_while.rs","permutations.rs","powerset.rs","process_results_impl.rs","put_back_n_impl.rs","rciter_impl.rs","repeatn.rs","size_hint.rs","sources.rs","tee.rs","tuple_impl.rs","unique_impl.rs","with_position.rs","zip_eq_impl.rs","zip_longest.rs","ziptuple.rs"]};
sourcesIndex["libm"] = {"name":"","dirs":[{"name":"math","files":["acos.rs","acosf.rs","acosh.rs","acoshf.rs","asin.rs","asinf.rs","asinh.rs","asinhf.rs","atan.rs","atan2.rs","atan2f.rs","atanf.rs","atanh.rs","atanhf.rs","cbrt.rs","cbrtf.rs","ceil.rs","ceilf.rs","copysign.rs","copysignf.rs","cos.rs","cosf.rs","cosh.rs","coshf.rs","erf.rs","erff.rs","exp.rs","exp10.rs","exp10f.rs","exp2.rs","exp2f.rs","expf.rs","expm1.rs","expm1f.rs","expo2.rs","fabs.rs","fabsf.rs","fdim.rs","fdimf.rs","fenv.rs","floor.rs","floorf.rs","fma.rs","fmaf.rs","fmax.rs","fmaxf.rs","fmin.rs","fminf.rs","fmod.rs","fmodf.rs","frexp.rs","frexpf.rs","hypot.rs","hypotf.rs","ilogb.rs","ilogbf.rs","j0.rs","j0f.rs","j1.rs","j1f.rs","jn.rs","jnf.rs","k_cos.rs","k_cosf.rs","k_expo2.rs","k_expo2f.rs","k_sin.rs","k_sinf.rs","k_tan.rs","k_tanf.rs","ldexp.rs","ldexpf.rs","lgamma.rs","lgamma_r.rs","lgammaf.rs","lgammaf_r.rs","log.rs","log10.rs","log10f.rs","log1p.rs","log1pf.rs","log2.rs","log2f.rs","logf.rs","mod.rs","modf.rs","modff.rs","nextafter.rs","nextafterf.rs","pow.rs","powf.rs","rem_pio2.rs","rem_pio2_large.rs","rem_pio2f.rs","remainder.rs","remainderf.rs","remquo.rs","remquof.rs","round.rs","roundf.rs","scalbn.rs","scalbnf.rs","sin.rs","sincos.rs","sincosf.rs","sinf.rs","sinh.rs","sinhf.rs","sqrt.rs","sqrtf.rs","tan.rs","tanf.rs","tanh.rs","tanhf.rs","tgamma.rs","tgammaf.rs","trunc.rs","truncf.rs"]}],"files":["lib.rs"]};
sourcesIndex["matrixmultiply"] = {"name":"","dirs":[{"name":"x86","files":["macros.rs","mod.rs"]}],"files":["aligned_alloc.rs","archparam_defaults.rs","debugmacros.rs","dgemm_kernel.rs","gemm.rs","kernel.rs","lib.rs","loopmacros.rs","ptr.rs","sgemm_kernel.rs","threading.rs","util.rs"]};
sourcesIndex["nalgebra"] = {"name":"","dirs":[{"name":"base","files":["alias.rs","alias_slice.rs","allocator.rs","array_storage.rs","blas.rs","blas_uninit.rs","cg.rs","componentwise.rs","constraint.rs","construction.rs","construction_slice.rs","conversion.rs","coordinates.rs","default_allocator.rs","dimension.rs","edition.rs","helper.rs","indexing.rs","interpolation.rs","iter.rs","matrix.rs","matrix_simba.rs","matrix_slice.rs","min_max.rs","mod.rs","norm.rs","ops.rs","properties.rs","scalar.rs","statistics.rs","storage.rs","swizzle.rs","uninit.rs","unit.rs","vec_storage.rs"]},{"name":"geometry","files":["abstract_rotation.rs","dual_quaternion.rs","dual_quaternion_construction.rs","dual_quaternion_conversion.rs","dual_quaternion_ops.rs","isometry.rs","isometry_alias.rs","isometry_construction.rs","isometry_conversion.rs","isometry_interpolation.rs","isometry_ops.rs","isometry_simba.rs","mod.rs","op_macros.rs","orthographic.rs","perspective.rs","point.rs","point_alias.rs","point_construction.rs","point_conversion.rs","point_coordinates.rs","point_ops.rs","point_simba.rs","quaternion.rs","quaternion_construction.rs","quaternion_conversion.rs","quaternion_coordinates.rs","quaternion_ops.rs","quaternion_simba.rs","reflection.rs","reflection_alias.rs","rotation.rs","rotation_alias.rs","rotation_construction.rs","rotation_conversion.rs","rotation_interpolation.rs","rotation_ops.rs","rotation_simba.rs","rotation_specialization.rs","similarity.rs","similarity_alias.rs","similarity_construction.rs","similarity_conversion.rs","similarity_ops.rs","similarity_simba.rs","swizzle.rs","transform.rs","transform_alias.rs","transform_construction.rs","transform_conversion.rs","transform_ops.rs","transform_simba.rs","translation.rs","translation_alias.rs","translation_construction.rs","translation_conversion.rs","translation_coordinates.rs","translation_ops.rs","translation_simba.rs","unit_complex.rs","unit_complex_construction.rs","unit_complex_conversion.rs","unit_complex_ops.rs","unit_complex_simba.rs"]},{"name":"linalg","files":["balancing.rs","bidiagonal.rs","cholesky.rs","col_piv_qr.rs","convolution.rs","decomposition.rs","determinant.rs","exp.rs","full_piv_lu.rs","givens.rs","hessenberg.rs","householder.rs","inverse.rs","lu.rs","mod.rs","permutation_sequence.rs","pow.rs","qr.rs","schur.rs","solve.rs","svd.rs","symmetric_eigen.rs","symmetric_tridiagonal.rs","udu.rs"]},{"name":"third_party","dirs":[{"name":"glam","files":["mod.rs"]}],"files":["mod.rs"]}],"files":["lib.rs"]};
sourcesIndex["nalgebra_macros"] = {"name":"","files":["lib.rs"]};
sourcesIndex["num_complex"] = {"name":"","files":["cast.rs","lib.rs","pow.rs"]};
sourcesIndex["num_integer"] = {"name":"","files":["average.rs","lib.rs","roots.rs"]};
sourcesIndex["num_rational"] = {"name":"","files":["lib.rs","pow.rs"]};
sourcesIndex["num_traits"] = {"name":"","dirs":[{"name":"ops","files":["checked.rs","inv.rs","mod.rs","mul_add.rs","overflowing.rs","saturating.rs","wrapping.rs"]}],"files":["bounds.rs","cast.rs","float.rs","identities.rs","int.rs","lib.rs","macros.rs","pow.rs","real.rs","sign.rs"]};
sourcesIndex["once_cell"] = {"name":"","files":["imp_std.rs","lib.rs","race.rs"]};
sourcesIndex["proc_macro2"] = {"name":"","files":["detection.rs","fallback.rs","lib.rs","marker.rs","parse.rs","wrapper.rs"]};
sourcesIndex["quote"] = {"name":"","files":["ext.rs","format.rs","ident_fragment.rs","lib.rs","runtime.rs","spanned.rs","to_tokens.rs"]};
sourcesIndex["rawpointer"] = {"name":"","files":["lib.rs"]};
sourcesIndex["safe_arch"] = {"name":"","dirs":[{"name":"x86_x64","files":["m128_.rs","m128d_.rs","m128i_.rs","m256_.rs","m256d_.rs","m256i_.rs","sse.rs","sse2.rs"]}],"files":["lib.rs","naming_conventions.rs"]};
sourcesIndex["sample_consensus"] = {"name":"","files":["lib.rs"]};
sourcesIndex["simba"] = {"name":"","dirs":[{"name":"scalar","files":["complex.rs","field.rs","mod.rs","real.rs","subset.rs"]},{"name":"simd","files":["auto_simd_impl.rs","mod.rs","simd_bool.rs","simd_complex.rs","simd_option.rs","simd_partial_ord.rs","simd_real.rs","simd_signed.rs","simd_value.rs","wide_simd_impl.rs"]}],"files":["lib.rs"]};
sourcesIndex["syn"] = {"name":"","dirs":[{"name":"gen","files":["clone.rs","debug.rs","eq.rs","gen_helper.rs","hash.rs","visit.rs"]}],"files":["attr.rs","await.rs","bigint.rs","buffer.rs","custom_keyword.rs","custom_punctuation.rs","data.rs","derive.rs","discouraged.rs","error.rs","export.rs","expr.rs","ext.rs","file.rs","generics.rs","group.rs","ident.rs","item.rs","lib.rs","lifetime.rs","lit.rs","lookahead.rs","mac.rs","macros.rs","op.rs","parse.rs","parse_macro_input.rs","parse_quote.rs","pat.rs","path.rs","print.rs","punctuated.rs","reserved.rs","sealed.rs","span.rs","spanned.rs","stmt.rs","thread.rs","token.rs","tt.rs","ty.rs","verbatim.rs","whitespace.rs"]};
sourcesIndex["typenum"] = {"name":"","files":["array.rs","bit.rs","int.rs","lib.rs","marker_traits.rs","operator_aliases.rs","private.rs","type_operators.rs","uint.rs"]};
sourcesIndex["unicode_xid"] = {"name":"","files":["lib.rs","tables.rs"]};
sourcesIndex["wide"] = {"name":"","files":["f32x4_.rs","f32x8_.rs","f64x2_.rs","f64x4_.rs","i16x16_.rs","i16x8_.rs","i32x4_.rs","i32x8_.rs","i64x2_.rs","i64x4_.rs","i8x16_.rs","i8x32_.rs","lib.rs","macros.rs","u16x8_.rs","u32x4_.rs","u32x8_.rs","u64x2_.rs","u64x4_.rs","u8x16_.rs"]};
createSourceSidebar();
