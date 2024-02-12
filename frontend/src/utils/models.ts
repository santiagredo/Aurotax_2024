export interface OnApproveData {
    billingToken?: string | null;
    facilitatorAccessToken: string;
    orderID: string;
    payerID?: string | null;
    paymentID?: string | null;
    subscriptionID?: string | null;
    authCode?: string | null;
}

export interface UserModel {
    userId: Number,
    userEmail: String,
    userName: String,
    paypalOrderId?: String,
}