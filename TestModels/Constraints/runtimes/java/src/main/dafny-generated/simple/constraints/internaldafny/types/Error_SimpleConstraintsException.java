// Class Error_SimpleConstraintsException
// Dafny class Error_SimpleConstraintsException compiled into Java
package simple.constraints.internaldafny.types;


@SuppressWarnings({"unchecked", "deprecation"})
public class Error_SimpleConstraintsException extends Error {
  public dafny.DafnySequence<? extends Character> _message;
  public Error_SimpleConstraintsException (dafny.DafnySequence<? extends Character> message) {
    super();
    this._message = message;
  }

  @Override
  public boolean equals(Object other) {
    if (this == other) return true;
    if (other == null) return false;
    if (getClass() != other.getClass()) return false;
    Error_SimpleConstraintsException o = (Error_SimpleConstraintsException)other;
    return true && java.util.Objects.equals(this._message, o._message);
  }
  @Override
  public int hashCode() {
    long hash = 5381;
    hash = ((hash << 5) + hash) + 0;
    hash = ((hash << 5) + hash) + java.util.Objects.hashCode(this._message);
    return (int)hash;
  }

  @Override
  public String toString() {
    StringBuilder s = new StringBuilder();
    s.append("SimpleConstraintsTypes.Error.SimpleConstraintsException");
    s.append("(");
    s.append(dafny.Helpers.toString(this._message));
    s.append(")");
    return s.toString();
  }
}
