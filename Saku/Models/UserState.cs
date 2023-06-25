namespace Saku.Models;

[Flags]
public enum UserState : ulong
{
    Normal = 0,
    Blocked = 1,
    TaxFree = 2,
    DmEnabled = 4,
    Virtual = 8,
}