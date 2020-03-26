export default class NavItem {
  public isActive: boolean = false;
  public text: string;
  public onClick: () => void;

  public withActive(active: boolean): NavItem {
    this.isActive = active;
    return this;
  }

  public withText(text: string): NavItem {
    this.text = text;
    return this;
  }

  public withOnClick(fn: () => void): NavItem {
    this.onClick = fn;
    return this;
  }
}
