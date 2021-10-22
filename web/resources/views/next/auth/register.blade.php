{{--
    Register screen (non-API).
    Extends from `next.base`.
--}}

@extends('next.base', [
    'title' => __('account_panel.REGISTER')
])

@push('scripts')
    @vite('auth-register.ts')
@endpush

@section('app')
    <div id="app_auth">
        <div id="auth_panel" class="light">
            <a href="/" title="{{ __('frame.GOTO_HOME_PAGE') }}">
                <img src="/files--static/media/logo.min.svg">
            </a>
            <hr>

            <div id="auth_form_container">
            </div>

            <a id="auth_login" href="/user--services/login">
                {{ __("account_panel.LOGIN") }}
            </a>

            {{-- gets placed _outside_ of the panel via styling --}}
            <div id="auth_links">
                {{-- TODO: link to actual pages --}}
                <a href="/terms">{{ __('frame.footer.TERMS') }}</a>
                <a href="/privacy">{{ __('frame.footer.PRIVACY') }}</a>
                <a href="/docs">{{ __('frame.footer.DOCS') }}</a>
                <a href="/security">{{ __('frame.footer.SECURITY') }}</a>
            </div>
        </div>
    </div>
@endsection