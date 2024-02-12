export const COOKIE_USER_ID = "__Aurotax_user_id__";
export const COOKIE_USER_NAME = "__Aurotax_user_name__";
export const COOKIE_USER_EMAIL = "__Aurotax_user_email__";
export const COOKIE_USER_IS_ADMIN = "__Aurotax_user_is_administrator__";
export const COOKIE_PAYPAL_ORDER_ID = "__Aurotax_paypal_order_id__";

export function getCookie(name: String) {
    let matches = document.cookie.match(
        new RegExp(
            "(?:^|; )" +
                name.replace(/([\.$?*|{}\(\)\[\]\\\/\+^])/g, "\\$1") +
                "=([^;]*)"
        )
    );
    return matches ? decodeURIComponent(matches[1]) : "";
}

export function deleteCookie(name: String) {
    document.cookie = `${name}=; expires=Thu, 01 Jan 1970 00:00:00 UTC; path=/;`;
}