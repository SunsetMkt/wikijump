{{--
    Register screen (non-API).
    Extends from `next.auth.auth`.
--}}

@extends('next.auth.auth', [
    'title' => __('auth.REGISTER')
])

@push('preloads')
    @preload('auth-register.ts')
@endpush

@push('scripts')
    @vite('auth-register.ts')
@endpush

@section('content')
    <div id="auth_form_container">
    </div>

    <a id="auth_login" href="/user--services/login">
        {{ __("auth.LOGIN") }}
    </a>
@endsection
