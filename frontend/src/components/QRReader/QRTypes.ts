namespace QRTypes {
    export interface Location {
        topRightCorner: {
            x: number;
            y: number;
        };
        topLeftCorner: {
            x: number;
            y: number;
        };
        bottomRightCorner: {
            x: number;
            y: number;
        };
        bottomLeftCorner: {
            x: number;
            y: number;
        };
        topRightFinderPattern: {
            x: number;
            y: number;
        };
        topLeftFinderPattern: {
            x: number;
            y: number;
        };
        bottomLeftFinderPattern: {
            x: number;
            y: number;
        };
        bottomRightAlignmentPattern: {
            x: number;
            y: number;
        };
    }
    export interface Detect {
        content: string;
        location: Location;
    }
}
export default QRTypes;
