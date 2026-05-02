export type AccessLevel = 'public' | 'premium';

export interface CreatorProfile {
    wallet: string;
    handle: string;
    bio: string;
    contentRef: string;
}

export interface ContentRef {
    id: number;
    creator: string;
    contentRef: string;
    access: AccessLevel;
}

export const hasActiveSubscription = (
    expiryUnix: number,
    nowUnix = Math.floor(Date.now() / 1000)
) => expiryUnix > nowUnix;
