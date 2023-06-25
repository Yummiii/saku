namespace Saku.Models;

[Flags]
public enum UserState : ulong
{
    Normal = 0,
    Blocked = 2,
    TaxFree = 4,
    DmEnabled = 8,
    Virtual = 16,
}