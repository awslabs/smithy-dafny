package software.amazon.polymorph.smithyjava;

public class NamespaceHelper {
    /**
     * Crypto Tools has used the namespace `aws.cryptography` in our Smithy Models;
     * <p>But AWS Java uses `software.amazon` instead of `aws`.*/
     // Historically, we have used `encryption` instead of `cryptography`,
     // but we are more flexible w.r.t. encryption vs cryptography.
    public static String standardize(String namespace) {
        String rtn = namespace;
        if (namespace.startsWith("aws")) {
            rtn = rtn.replaceFirst("aws", "software.amazon");
        }
        return rtn;
    }
}
