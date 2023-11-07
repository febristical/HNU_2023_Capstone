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

#bigtitle(alignment: horizon + center, "Ray Marching 을 이용한 3D 렌더링")

#verifier("한남대학교 수학과", "20172581 김남훈")

#pagebreak()

