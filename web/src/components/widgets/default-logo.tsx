import { Image, type ImageProps } from "../ui/image";

type DefaultLogoProps = ImageProps & {};

function DefaultLogo(props: DefaultLogoProps) {
  return (
    <Image src={"/logo.svg"} alt={"CdsCTF Logo"} delay={0} glass={false} {...props} />
  );
}

export { DefaultLogo, type DefaultLogoProps };
