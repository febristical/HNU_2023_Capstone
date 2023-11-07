#set text(font: "KoPubBatang_Pro", size: 18pt)
#set page(width: 960pt, height: 540pt, margin: 7.5%)

#let bigtitle(alignment: center, body) = align(
    alignment,
    text(font: "KoPubDotum_Pro", weight: "black", size: 54pt, body)
)

#let verifier(school, author) = align(
    bottom + right,
    stack(
        dir:ttb,
        spacing: 10pt,
        text(size: 20pt, school),
        text(size: 20pt, author)
    )
)

#let title(alignment: center, body) = align(
    alignment,
    text(font: "KoPubDotum_Pro", weight: "black", size: 36pt, body)
)

#let contentbox(nameweight: "bold", name, primary, secondary) = rect(
    width: 100%,
    fill: rgb("F0F0F0"),
    radius: 24pt,
    outset: 10pt,
    stack(
        dir:ttb,
        spacing: 24pt,
        text(weight: nameweight, name + ".") + h(5pt) + primary,
        secondary
    )
)

#bigtitle(alignment: horizon + center, "테스트")

#verifier("테스트", "테스트")

#pagebreak()

#contentbox("정의", "안녕",
    [
        세계 $integral$
    ]
)