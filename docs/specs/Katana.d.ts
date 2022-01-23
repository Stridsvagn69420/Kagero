export interface Katana {
    uuid: Uuid;
    website: string;
    root: string;
    name: string;
    maintainers: Maintainer[];
    shurikens: string[];
}

type Uuid = '/^[0-9A-F]{8}-[0-9A-F]{4}-4[0-9A-F]{3}-[89AB][0-9A-F]{3}-[0-9A-F]{12}$/i';

type Maintainer = {
    name: string;
    email: string;
    profile: string;
}