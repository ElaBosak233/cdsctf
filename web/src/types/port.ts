export interface Port {
    value: number;
    protocol: Protocol;
}

enum Protocol {
    TCP = 0,
    UDP = 1,
}
