import React from "react";
import { useNavigate } from "react-router-dom";

export default function TestPage() {
    const navigate = useNavigate();

    const checkOrderExistence = async () => {
        const request = await fetch(
            `http://127.0.0.1:8080/authentication/verify_order_existence`,
            {
                method: "POST",
                credentials: "include",
                mode: "cors",
                headers: {
                    "Content-Type": "application/json",
                    "Access-Control-Allow-Credentials": "true",
                },
            }
        );

        if (request.ok) {
            alert("Ok");
            navigate("/test2");
        } else if (request.status === 500) {
            alert("Server Error");
            navigate("/");
        } else if (request.status === 401) {
            alert("Unauthorized");
            navigate("/paypal");
        }
    };

    React.useEffect(() => {
        checkOrderExistence();
    }, []);

    return <h1 className="text-red-700 text-3xl">This is a test page</h1>;
}
