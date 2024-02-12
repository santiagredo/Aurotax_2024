import { getCookie } from "../utils/cookies";

export default function AdminPanel() {
    return (
        <main>
            <h1>Administration Panel</h1>

            <h2>
                {getCookie("__Aurotax_user_email__")}
            </h2>

            <h2>{getCookie("__Aurotax_paypal_order_id__")}</h2>
        </main>
    );
}
